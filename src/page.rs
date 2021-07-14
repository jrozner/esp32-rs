use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct Page {
    state: State,
    seq_no: u32,
    version: u8,
    unused: Vec<u8>,
    crc32: u32,
    entry_state_bitmap: Vec<EntryStateBitmap>,
    data: Vec<u8>,
}

impl Page {
    pub fn new(
        state: State,
        seq_no: u32,
        version: u8,
        unused: Vec<u8>,
        crc32: u32,
        entry_state_bitmap: Vec<EntryStateBitmap>,
        data: Vec<u8>,
    ) -> Page {
        Page {
            state,
            seq_no,
            version,
            unused,
            crc32,
            entry_state_bitmap,
            data,
        }
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn entry_state_bitmap(&self) -> &[EntryStateBitmap] {
        &self.entry_state_bitmap
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

#[derive(Debug, Clone)]
pub enum State {
    Empty,
    Active,
    Full,
    Erasing,
    Corrupted,
}

impl TryFrom<u32> for State {
    // TODO: use a real error type here
    type Error = ();

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        match val {
            0xfffffff0 => Ok(Self::Corrupted),
            0xfffffff8 => Ok(Self::Erasing),
            0xfffffffc => Ok(Self::Full),
            0xfffffffe => Ok(Self::Active),
            0xffffffff => Ok(Self::Empty),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntryStateBitmap {
    Empty,
    Written,
    Erased,
}

impl TryFrom<u8> for EntryStateBitmap {
    type Error = InvalidBitmapError;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Erased),
            2 => Ok(Self::Written),
            3 => Ok(Self::Empty),
            _ => Err(InvalidBitmapError { value: val }),
        }
    }
}

// TODO: implement std::error::Error
#[derive(Debug, Clone)]
pub struct InvalidBitmapError {
    value: u8,
}
