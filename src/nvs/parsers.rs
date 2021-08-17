use std::convert::TryFrom;
use std::ffi::CString;

use nom::combinator::map_res;
use nom::multi::count;
use nom::number::complete::{le_i16, le_i32, le_i64, le_i8, le_u16, le_u32, le_u64, le_u8};
use nom::sequence::tuple;
use nom::IResult;

use crate::nvs::event::{Entry, EntryType};
use crate::nvs::page::{EntryStateBitmap, Page};

pub(crate) fn page(input: &[u8]) -> IResult<&[u8], Page> {
    let (input, state) = map_res(le_u32, |val| crate::nvs::page::State::try_from(val))(input)?;
    let (input, seq_no) = le_u32(input)?;
    let (input, version) = le_u8(input)?;
    let (input, unused) = count(le_u8, 19)(input)?;
    let (input, crc32) = le_u32(input)?;
    let (input, bitmaps_raw) = count(le_u32, 8)(input)?;
    let mut entry_state_bitmap = vec![];

    // NOTE: this will have 2 extra entries (128) because 32 bytes are used to store the bitmaps
    for word in bitmaps_raw {
        for i in 0..16 {
            let val = ((word >> (i * 2)) & 0x3) as u8;
            let entry = EntryStateBitmap::try_from(val).unwrap();
            entry_state_bitmap.push(entry);
        }
    }

    // TODO: eventually get back to a bit based parsing when we have a way
    // to handle le_u32 packed bits so we can have good error handling
    //let (input, entry_state_bitmap) =
    //    bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(count(
    //        map_res(take(2u8), |byte: u8| EntryStateBitmap::try_from(byte)),
    //        126,
    //    ))(input)?;
    let (input, data) = count(le_u8, 32 * 126)(input)?;

    Ok((
        input,
        Page::new(
            state,
            seq_no,
            version,
            unused,
            crc32,
            entry_state_bitmap,
            data,
        ),
    ))
}

pub(crate) fn entry(input: &[u8], page: u8, start: u8) -> IResult<&[u8], Entry> {
    let (input, ns) = le_u8(input)?;
    let (input, entry_type) = le_u8(input)?;
    let (input, span) = le_u8(input)?;
    let (input, chunk_index) = le_u8(input)?;
    let (input, crc32) = le_u32(input)?;
    let (input, key_raw) = count(le_u8, 16)(input)?;
    let first_null = key_raw.iter().position(|b| *b == 0).unwrap();
    let str_bytes = key_raw[0..first_null].to_owned();
    let key = CString::new(str_bytes).unwrap().into_string().unwrap();

    let (input, data) = match entry_type {
        0x1 => {
            let (input, data) = le_u8(input)?;
            let (input, _) = count(le_u8, 7)(input)?;
            (input, EntryType::U8(data))
        }
        0x2 => {
            let (input, data) = le_u16(input)?;
            let (input, _) = count(le_u8, 6)(input)?;
            (input, EntryType::U16(data))
        }
        0x4 => {
            let (input, data) = le_u32(input)?;
            let (input, _) = count(le_u8, 4)(input)?;
            (input, EntryType::U32(data))
        }
        0x8 => {
            let (input, data) = le_u64(input)?;
            (input, EntryType::U64(data))
        }
        0x11 => {
            let (input, data) = le_i8(input)?;
            let (input, _) = count(le_u8, 7)(input)?;
            (input, EntryType::I8(data))
        }
        0x12 => {
            let (input, data) = le_i16(input)?;
            let (input, _) = count(le_u8, 6)(input)?;
            (input, EntryType::I16(data))
        }
        0x14 => {
            let (input, data) = le_i32(input)?;
            let (input, _) = count(le_u8, 4)(input)?;
            (input, EntryType::I32(data))
        }
        0x18 => {
            let (input, data) = le_i64(input)?;
            (input, EntryType::I64(data))
        }
        0x21 => unimplemented!(),
        0x41 => {
            // legacy style blobs where data is stored directly after
            let (input, (size, _, crc32)) = tuple((le_u16, le_u16, le_u32))(input)?;
            let rounded_size = (size + 32 - 1) & !(32 - 1);
            let (input, data) = count(le_u8, size as usize)(input)?;
            let padding = rounded_size - size;
            let (input, _) = count(le_u8, padding as usize)(input)?;

            (input, EntryType::Blob(data))
        }
        0x42 => unimplemented!(),
        0x48 => unimplemented!(),
        0xff => (input, EntryType::Any),
        _ => unimplemented!(),
    };

    Ok((
        input,
        Entry::new(
            ns,
            span,
            chunk_index,
            crc32,
            key,
            data,
            page,
            start,
            start + span,
        ),
    ))
}
