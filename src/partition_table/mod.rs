use std::fs::File;
use std::io::Read;
use std::fmt::Formatter;

use nom::bytes::complete::{tag, take};
use nom::combinator::map;
use nom::multi::many_m_n;
use nom::number::complete::{le_u32, le_u8};
use nom::{IResult, InputIter};
use serde::Serialize;

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

fn parse_hash(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    let (input, hash) = map(take(32usize), |bytes: &[u8]| bytes.to_vec())(input)?;
    Ok((input, hash))
}

#[derive(Debug, Clone)]
pub struct PartitionTable {
    partitions: Vec<Partition>,
    hash: Vec<u8>,
}

impl PartitionTable {
    pub fn new(input: &[u8]) -> PartitionTable {
        let (input, partitions) = many_m_n(1, 95, parse_partition)(input).unwrap();
        let (_, hash) = parse_hash(input).unwrap();

        PartitionTable { partitions, hash }
    }

    pub fn from_file(filename: &str) -> PartitionTable {
        let mut file = File::open(filename).unwrap();
        let mut data = vec![];
        file.read_to_end(&mut data).unwrap();

        PartitionTable::new(&data)
    }

    pub fn partitions(&self) -> &[Partition] {
        &self.partitions
    }

    pub fn hash(&self) -> &[u8] {
        &self.hash
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Partition {
    name: String,
    #[serde(rename="type")]
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

#[derive(Debug, Clone, Serialize)]
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

impl std::fmt::Display for PartitionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::App => write!(f, "app"),
            Self::Data => write!(f, "data"),
            Self::Any => write!(f, "{:x}", 255),
            Self::Custom(i) | Self::Invalid(i) => write!(f, "{:x}", i),
        }
    }
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

#[derive(Debug, Clone, Serialize)]
pub enum Subtype {
    #[serde(rename="factory")]
    AppFactory,
    #[serde(rename="ota0")]
    AppOta0,
    #[serde(rename="ota1")]
    AppOta1,
    #[serde(rename="ota2")]
    AppOta2,
    #[serde(rename="ota3")]
    AppOta3,
    #[serde(rename="ota4")]
    AppOta4,
    #[serde(rename="ota5")]
    AppOta5,
    #[serde(rename="ota6")]
    AppOta6,
    #[serde(rename="ota7")]
    AppOta7,
    #[serde(rename="ota8")]
    AppOta8,
    #[serde(rename="ota9")]
    AppOta9,
    #[serde(rename="ota10")]
    AppOta10,
    #[serde(rename="ota11")]
    AppOta11,
    #[serde(rename="ota12")]
    AppOta12,
    #[serde(rename="ota13")]
    AppOta13,
    #[serde(rename="ota14")]
    AppOta14,
    #[serde(rename="ota15")]
    AppOta15,
    #[serde(rename="test")]
    AppTest,
    #[serde(rename="ota")]
    DataOta,
    #[serde(rename="phy")]
    DataPhy,
    #[serde(rename="nvs")]
    DataNvs,
    #[serde(rename="coredump")]
    DataCoreDump,
    #[serde(rename="nvs_keys")]
    DataNvsKeys,
    #[serde(rename="efuse")]
    DataEfuse,
    #[serde(rename="esphttpd")]
    DataEspHttpd,
    #[serde(rename="fat")]
    DataFat,
    #[serde(rename="spiffs")]
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

impl std::fmt::Display for Subtype {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AppFactory => write!(f, "factory"),
            Self::AppOta0 => write!(f, "ota_0"),
            Self::AppOta1=> write!(f, "ota_1"),
            Self::AppOta2=> write!(f, "ota_2"),
            Self::AppOta3=> write!(f, "ota_3"),
            Self::AppOta4 => write!(f, "ota_4"),
            Self::AppOta5=> write!(f, "ota_5"),
            Self::AppOta6=> write!(f, "ota_6"),
            Self::AppOta7 => write!(f, "ota_7"),
            Self::AppOta8 => write!(f, "ota_8"),
            Self::AppOta9=> write!(f, "ota_9"),
            Self::AppOta10=> write!(f, "ota_10"),
            Self::AppOta11 => write!(f, "ota_11"),
            Self::AppOta12 => write!(f, "ota_12"),
            Self::AppOta13=> write!(f, "ota_13"),
            Self::AppOta14=> write!(f, "ota_14"),
            Self::AppOta15=> write!(f, "ota_15"),
            Self::AppTest=> write!(f, "test"),
            Self::DataOta => write!(f, "ota"),
            Self::DataPhy => write!(f, "phy"),
            Self::DataNvs=> write!(f, "nvs"),
            Self::DataCoreDump=> write!(f, "coredump"),
            Self::DataNvsKeys=> write!(f, "nvs_keys"),
            Self::DataEfuse => write!(f, "efuse"),
            Self::DataEspHttpd => write!(f, "esphttpd"),
            Self::DataFat=> write!(f, "fat"),
            Self::DataSpiffs=> write!(f, "spiffs"),
            Self::Any=> write!(f, "0xff"),
            Self::Invalid(i) => write!(f, "{:x}", i),
            Self::Custom(i) => write!(f, "{:x}", i),
        }
    }
}
