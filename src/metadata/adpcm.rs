use std::collections::HashMap;

use memchr::memmem;

use super::meta::{Metadata, MetadataScanner};

pub struct Scanner {}

impl MetadataScanner for Scanner {
    fn name(&self) -> &'static str {
        "ADPCM"
    }

    fn scan(&self, index: usize, file: &crate::kidz::KidzFile) -> HashMap<String, Metadata> {
        let upper_delimiter = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        let bottom_delimiter = b"\x00\x07\x77\x77\x77\x77\x77\x77\x77\x77\x77\x77\x77\x77\x77\x77";

        let mut r = HashMap::new();

        // TODO: This could be better
        if let Some(upper_pos) = memmem::find(&file.data, upper_delimiter) {
            if let Some(_pos) = memmem::find(&file.data[upper_pos + 16..], bottom_delimiter) {
                r.insert("ADPCM".into(), Metadata::Adpcm);
            }
        }

        r
    }
}
