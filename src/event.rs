use std::fmt::Formatter;

/// This is a high level abstraction that represents a full entry stored in
/// an nvs partition. This is opposed to an `entry` in the sense of nvs which
/// is simply a 32 byte block of data within a page or spread across pages.
/// This entry type will track the page(s) and nvs entries that back in on
/// the partition.
#[derive(Debug, Clone)]
pub struct Entry {
    ns: u8,
    span: u8,
    chunk_index: u8,
    crc32: u32,
    key: String,
    data: EntryType,
    page: u8,
    start: u8,
    end: u8,
}

impl Entry {
    pub fn new(
        ns: u8,
        span: u8,
        chunk_index: u8,
        crc32: u32,
        key: String,
        data: EntryType,
        page: u8,
        start: u8,
        end: u8,
    ) -> Entry {
        Entry {
            ns,
            span,
            chunk_index,
            crc32,
            key,
            data,
            page,
            start,
            end,
        }
    }

    pub fn ns(&self) -> u8 {
        self.ns
    }

    pub fn span(&self) -> u8 {
        self.span
    }

    pub fn chunk_index(&self) -> u8 {
        self.chunk_index
    }

    pub fn crc32(&self) -> u32 {
        self.crc32
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn data(&self) -> &EntryType {
        &self.data
    }

    pub fn page(&self) -> u8 {
        self.page
    }

    pub fn start(&self) -> u8 {
        self.start
    }

    pub fn end(&self) -> u8 {
        self.end
    }
}

/// Representation of all types within the nvs spec mapped into Rust types
#[derive(Debug, Clone)]
pub enum EntryType {
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    String(String),
    Blob(Vec<u8>),
    BlobData(Vec<u8>),
    BlobIndex,
    Any,
}

impl std::fmt::Display for EntryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U8(val) => write!(f, "{}", val),
            Self::I8(val) => write!(f, "{}", val),
            Self::U16(val) => write!(f, "{}", val),
            Self::I16(val) => write!(f, "{}", val),
            Self::U32(val) => write!(f, "{}", val),
            Self::I32(val) => write!(f, "{}", val),
            Self::U64(val) => write!(f, "{}", val),
            Self::I64(val) => write!(f, "{}", val),
            Self::String(val) => write!(f, "{}", val),
            Self::Blob(val) => write!(f, "{:?}", val),
            Self::BlobData(_) => unimplemented!(),
            Self::BlobIndex => unimplemented!(),
            Self::Any => unreachable!(),
        }
    }
}
