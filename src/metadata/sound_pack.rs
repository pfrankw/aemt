use std::collections::HashMap;

use super::meta::{Metadata, MetadataScanner};

pub struct Scanner {}

impl MetadataScanner for Scanner {
    fn name(&self) -> &'static str {
        "sound_pack"
    }

    fn scan(&self, index: usize, _file: &crate::kidz::KidzFile) -> HashMap<String, Metadata> {
        let mut r = HashMap::new();

        if index == 10 {
            r.insert("type".into(), Metadata::SoundPack);
        }

        r
    }
}
