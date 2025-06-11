use std::collections::HashMap;

use memchr::memmem;

use super::meta::{Metadata, MetadataScanner};

pub struct Scanner {}

impl MetadataScanner for Scanner {
    fn name(&self) -> &'static str {
        "SShd"
    }

    fn scan(&self, index: usize, file: &crate::kidz::KidzFile) -> HashMap<String, Metadata> {
        let mut r = HashMap::new();

        if let Some(_pos) = memmem::find(&file.data, b"SShd") {
            r.insert("SShd".into(), Metadata::Sshd);
        }

        r
    }
}
