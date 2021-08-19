use std::fs::File;
use std::io::Read;

use nom::bytes::complete::{tag, take};
use nom::combinator::map;
use nom::multi::many_m_n;
use nom::number::complete::{le_u32, le_u8};
use nom::{IResult, InputIter};

fn parse_partition(input: &[u8]) -> IResult<&[u8], Partition> {
    let (input, _) = tag(&[0xaa, 0x50])(input)?;
    let (input, partition_type) = map(le_u8, |val: u8| PartitionType::from(val))(input)?;
    let (input, subtype) = map(le_u8, |subtype| Subtype::new(&partition_type, subtype))(input)?;
    let (input, offset) = le_u32(input)?;
    let (input, size) = le_u32(input)?;
    let (input, name) = map(take(16usize), |bytes: &[u8]| {
        let end = bytes.position(|c| c == 0);
        match end {
            Some(end) => String::from_utf8_lossy(&bytes[0..end]).to_string(),
            None => String::from(""),
        }
    })(input)?;
    let (input, flags) = le_u32(input)?;
    let partition = Partition::new(name, partition_type, subtype, offset, size, flags);
    Ok((input, partition))
}

#[derive(Debug, Clone)]
pub struct PartitionTable {
    partitions: Vec<Partition>,
    hash: [u8; 32],
}

impl PartitionTable {
    pub fn new(input: &[u8]) -> PartitionTable {
        let (_, partitions) = many_m_n(1, 95, parse_partition)(&input).unwrap();

        PartitionTable {
            partitions,
            hash: [0; 32],
        }
    }

    pub fn from_file(filename: &str) -> PartitionTable {
        let mut file = File::open(filename).unwrap();
        let mut data = vec![];
        file.read_to_end(&mut data).unwrap();

        PartitionTable::new(&data)
    }
}

#[derive(Debug, Clone)]
pub struct Partition {
    name: String,
    partition_type: PartitionType,
    subtype: Subtype,
    offset: u32,
    size: u32,
    flags: u32,
}

impl Partition {
    pub fn new(
        name: String,
        partition_type: PartitionType,
        subtype: Subtype,
        offset: u32,
        size: u32,
        flags: u32,
    ) -> Partition {
        Partition {
            name,
            partition_type,
            subtype,
            offset,
            size,
            flags,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PartitionType {
    App,
    Data,
    Any,
    /// Custom partition types can be added to be used for partitions not part of the core esp-idf.
    /// These can be in the range of 64-255
    Custom(u8),
    /// The esp-idf reserves values 0-64 for core functions. Invalid is used to catch anything in
    /// this range that does not have a defined value
    Invalid(u8),
}

impl From<u8> for PartitionType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::App,
            1 => Self::Data,
            64..=254 => Self::Custom(value),
            255 => Self::Any,
            _ => Self::Invalid(value),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Subtype {
    AppFactory,
    AppOta0,
    AppOta1,
    AppOta2,
    AppOta3,
    AppOta4,
    AppOta5,
    AppOta6,
    AppOta7,
    AppOta8,
    AppOta9,
    AppOta10,
    AppOta11,
    AppOta12,
    AppOta13,
    AppOta14,
    AppOta15,
    AppTest,
    DataOta,
    DataPhy,
    DataNvs,
    DataCoreDump,
    DataNvsKeys,
    DataEfuse,
    DataEspHttpd,
    DataFat,
    DataSpiffs,
    Any,
    Invalid(u8),
    Custom(u8),
}

impl Subtype {
    pub fn new(partition_type: &PartitionType, subtype_value: u8) -> Subtype {
        match partition_type {
            PartitionType::App => match subtype_value {
                0 => Subtype::AppFactory,
                16 => Subtype::AppOta0,
                17 => Subtype::AppOta1,
                18 => Subtype::AppOta2,
                19 => Subtype::AppOta3,
                20 => Subtype::AppOta4,
                21 => Subtype::AppOta5,
                22 => Subtype::AppOta6,
                23 => Subtype::AppOta7,
                24 => Subtype::AppOta8,
                25 => Subtype::AppOta9,
                26 => Subtype::AppOta10,
                27 => Subtype::AppOta11,
                28 => Subtype::AppOta12,
                29 => Subtype::AppOta13,
                30 => Subtype::AppOta14,
                31 => Subtype::AppOta15,
                32 => Subtype::AppTest,
                _ => Subtype::Invalid(subtype_value),
            },
            PartitionType::Data => match subtype_value {
                0 => Subtype::DataOta,
                1 => Subtype::DataPhy,
                2 => Subtype::DataNvs,
                3 => Subtype::DataCoreDump,
                4 => Subtype::DataNvsKeys,
                5 => Subtype::DataEfuse,
                128 => Subtype::DataEspHttpd,
                129 => Subtype::DataFat,
                130 => Subtype::DataSpiffs,
                _ => Subtype::Invalid(subtype_value),
            },
            PartitionType::Custom(_) => match subtype_value {
                0..=254 => Subtype::Custom(subtype_value),
                255 => Subtype::Any,
            },
            PartitionType::Invalid(_) => Subtype::Invalid(subtype_value),
            PartitionType::Any => Subtype::Any,
        }
    }
}
