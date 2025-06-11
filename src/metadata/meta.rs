use std::{collections::HashMap, fmt::Display};

use crate::kidz::KidzFile;

use super::{sound_pack, sound_pack_ptr};

pub enum Metadata {
    Generic(String),
    SoundPack,
    SoundPackPtr,
    Str(String),
}

impl Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = match self {
            Metadata::Generic(m) => format!("Generic: {}", m),
            Metadata::SoundPack => "Sound Pack".into(),
            Metadata::SoundPackPtr => "Sound Pack Pointers".into(),
            Metadata::Str(name) => format!("STR Pointer: {}", name),
        };

        r.fmt(f)
    }
}

pub trait MetadataScanner {
    fn name(&self) -> &'static str;

    fn scan(&self, index: usize, file: &KidzFile) -> HashMap<String, Metadata>;
}

/// Metadata Scanner Registry
pub struct Msr {
    scanners: Vec<(String, Box<dyn MetadataScanner>)>,
}

impl Msr {
    pub fn new() -> Self {
        let mut msr = Self { scanners: vec![] };

        msr.register_all();

        msr
    }

    pub fn scan(&self, index: usize, file: &KidzFile) -> HashMap<String, Metadata> {
        let mut r = HashMap::new();

        for scanner in &self.scanners {
            let h = scanner.1.scan(index, file);
            r.extend(h);
        }

        r
    }

    fn register<S: MetadataScanner + 'static>(&mut self, scanner: S) {
        self.scanners
            .push((scanner.name().into(), Box::new(scanner)));
    }

    fn register_all(&mut self) {
        self.register(sound_pack::Scanner {});
        self.register(sound_pack_ptr::Scanner {});
    }
}
