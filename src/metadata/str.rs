use std::collections::HashMap;

use super::meta::{Metadata, MetadataScanner};

pub struct Scanner {}

impl MetadataScanner for Scanner {
    fn name(&self) -> &'static str {
        "STR"
    }

    fn scan(&self, index: usize, _file: &crate::kidz::KidzFile) -> HashMap<String, Metadata> {
        let mut r = HashMap::new();

        let m = match index {
            448 => "LAB.STR",
            449 => "WEPGET.STR",
            450 => "GOV1.STR",
            451 => "GOV2.STR",
            452 => "STEXP.STR",
            453 => "TALK.STR",
            454 => "BUILDD1.STR",
            455 => "BUILDD2.STR",
            456 => "A_HA_D1.STR",
            457 => "A_HA_D2.STR",
            458 => "A_NA_D1.STR",
            459 => "A_NA_D2.STR",
            460 => "A_RAI_D1.STR",
            461 => "A_RAI_D2.STR",
            462 => "A_SPE_D1.STR",
            463 => "SFIN_D1.STR",
            464 => "SFIN_D2.STR",
            465 => "SFIN_D3.STR",
            466 => "CASTLE.STR",
            467 => "FINAL_D1.STR",
            _ => "",
        };

        if !m.is_empty() {
            r.insert("STR".into(), Metadata::Str(m.into()));
        }

        r
    }
}
