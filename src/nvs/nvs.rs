use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use nom::multi::many0;

use crate::nvs::event::{Entry, EntryType};
use crate::nvs::page::{EntryStateBitmap, Page};

#[derive(Debug, Clone)]
pub struct Nvs {
    pages: Vec<Page>,
    entries: Vec<Entry>,
    namespace_lookup: HashMap<u8, String>,
    namespaces: HashMap<u8, Vec<usize>>,
    name_to_ns: HashMap<String, u8>,
    use_deleted: bool,
}

impl Nvs {
    pub fn new(file: &str, use_deleted: bool) -> Nvs {
        let mut f = File::open(file).unwrap();
        let mut data = vec![];
        f.read_to_end(&mut data).unwrap();

        let (_, pages) = many0(crate::nvs::parsers::page)(&data).unwrap();

        let mut entries = vec![];
        let mut namespaces: HashMap<u8, Vec<usize>> = HashMap::new();
        let mut namespace_lookup: HashMap<u8, String> = HashMap::new();
        let mut name_to_ns = HashMap::new();

        for (i, page) in pages.iter().enumerate() {
            let mut page_data = page.data();
            let mut start = 0;
            let bitmaps = page.entry_state_bitmap();
            while page_data.len() > 0 {
                if bitmaps[start] == EntryStateBitmap::Empty
                    || (bitmaps[start] == EntryStateBitmap::Erased && !use_deleted)
                {
                    start += 1;
                    page_data = &page_data[32..];
                    continue;
                }

                let (remainder, entry) =
                    crate::nvs::parsers::entry(page_data, i as u8, start as u8).unwrap();
                page_data = remainder;
                start = entry.end() as usize;

                if entry.ns() == 0 {
                    let key = match entry.data() {
                        EntryType::U8(val) => val,
                        _ => panic!("invalid type for namespace"),
                    };

                    if let EntryType::U8(ns_id) = entry.data() {
                        namespace_lookup.insert(*ns_id, entry.key().to_owned());
                        namespaces.insert(*ns_id, vec![]);
                        name_to_ns.insert(entry.key().to_owned(), *ns_id);
                        continue;
                    } else {
                        panic!("invalid type for ns");
                    }
                } else {
                    entries.push(entry);
                    match namespaces.get_mut(&entries.last().unwrap().ns()) {
                        Some(ns) => ns.push(entries.len() - 1),
                        None => println!("ns {} does not exist", entries.last().unwrap().key()),
                    }
                }
            }
        }

        Nvs {
            pages,
            entries,
            namespace_lookup,
            namespaces,
            name_to_ns,
            use_deleted,
        }
    }

    pub fn namespaces(&self) -> Vec<&str> {
        self.namespace_lookup.values().map(|v| v.as_str()).collect()
    }

    pub fn namespace(&self, ns: &str) -> Option<HashMap<&str, &Entry>> {
        let ns_idx = match self.name_to_ns.get(ns) {
            Some(idx) => idx,
            None => return None,
        };

        match self.namespaces.get(ns_idx) {
            Some(entry_ids) => {
                let mut namespace = HashMap::new();
                for idx in entry_ids {
                    let entry = &self.entries[*idx];
                    namespace.insert(entry.key(), entry);
                }
                Some(namespace)
            }
            None => None,
        }
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    pub fn pages(&self) -> &[Page] {
        &self.pages
    }
}
