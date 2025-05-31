use std::{array::TryFromSliceError, fmt, fs::File, os::unix::fs::FileExt, path::Path};

#[derive(Default, Clone, Copy)]
pub struct HedEntry {
    /// 20-bit integer
    pub offset: u32,

    /// 12-bit integer
    pub len: u16,
}

impl From<u32> for HedEntry {
    fn from(value: u32) -> Self {
        Self {
            offset: value & 0xFFFFF,
            len: (value >> 20) as u16,
        }
    }
}

impl From<[u8; 4]> for HedEntry {
    fn from(value: [u8; 4]) -> Self {
        Self::from(u32::from_le_bytes(value))
    }
}

impl TryFrom<&[u8]> for HedEntry {
    type Error = TryFromSliceError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::from(u32::from_le_bytes(value.try_into()?)))
    }
}

impl From<HedEntry> for u32 {
    fn from(value: HedEntry) -> Self {
        ((value.len as u32) << 20) | value.offset
    }
}

impl From<HedEntry> for [u8; 4] {
    fn from(value: HedEntry) -> Self {
        u32::from(value).to_le_bytes()
    }
}

#[derive(Clone, Copy)]
pub enum KidzFileType {
    Empty,
    Dat,
    Bns,
    Str,
}

impl std::fmt::Display for KidzFileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ret = match self {
            KidzFileType::Str => "STR",
            KidzFileType::Dat => "DAT",
            KidzFileType::Bns => "BNS",
            KidzFileType::Empty => "Empty",
        };

        ret.fmt(f)
    }
}

pub struct KidzFile {
    pub hed: HedEntry,
    pub data: Vec<u8>,
    pub t: KidzFileType,
}

impl KidzFile {}

pub struct Kidz {
    pub files: Vec<KidzFile>,
}

impl Kidz {
    fn read_all_files(
        entries: &[HedEntry],
        dat: &File,
        bns: &File,
    ) -> Result<Vec<KidzFile>, crate::error::Error> {
        let mut files = vec![];
        let dat_len = dat.metadata()?.len();

        for (i, entry) in entries.iter().enumerate() {
            // Checking if it is an empty file
            match entry.len {
                // No len == empty file
                0 => {
                    files.push(KidzFile {
                        hed: *entry,
                        data: vec![],
                        t: KidzFileType::Empty,
                    });
                }
                // Any other file is non-empty
                _ => {
                    let full_len = entry.len as usize * 2048;
                    let position = entry.offset as u64 * 2048;
                    let mut data = vec![0u8; full_len];
                    let filetype;

                    // Checking if we are reading an STR file
                    // STR files do not actually get read. Only their indexes get stored to be
                    // adjusted later in case of operations
                    match i < (0x700 / 4) {
                        // If position is > DAT Length it means that the current file is located in
                        // the BNS file
                        true => match position < dat_len {
                            true => {
                                dat.read_exact_at(&mut data, position)
                                    .or(Err("Failed to read DAT"))?;

                                filetype = KidzFileType::Dat;
                            }

                            // If position is over the DAT file it means we need to read from the
                            // BNS file
                            false => {
                                let position = position - dat_len;

                                bns.read_exact_at(&mut data, position)
                                    .or(Err("Failed to read BNS"))?;

                                filetype = KidzFileType::Bns;
                            }
                        },
                        // If > 700h we are reading an STR file
                        false => filetype = KidzFileType::Str,
                    }

                    files.push(KidzFile {
                        hed: *entry,
                        data,
                        t: filetype,
                    });
                }
            }
        }

        Ok(files)
    }

    pub fn load<P: AsRef<Path>>(hed: P, dat: P, bns: P) -> Result<Self, crate::error::Error> {
        let hed = File::open(hed)?;
        let dat = File::open(dat)?;
        let bns = File::open(bns)?;

        let mut buf = [0u8; 0x800];

        hed.read_exact_at(&mut buf, 0)
            .or(Err("Failed to read STR section of HED"))?;

        let entries: Result<Vec<HedEntry>, TryFromSliceError> =
            buf.chunks_exact(4).map(HedEntry::try_from).collect();
        let entries = entries?;

        Ok(Self { files: Self::read_all_files(&entries, &dat, &bns)? })
    }
}

#[test]
fn test_hed_entry() {
    let entry_input = 0x07F000E0u32;
    let entry = HedEntry::from(entry_input);

    assert!(entry.offset == 224);
    assert!(entry.len == 127);

    let entry: u32 = entry.into();

    assert!(entry == entry_input);
}

#[test]
fn test_hed_entry_slice() {
    let slice_input = [0xE0, 0x00, 0xF0, 0x07u8];
    let entry = HedEntry::from(slice_input);

    assert!(entry.offset == 224);
    assert!(entry.len == 127);

    let slice: [u8; 4] = entry.into();

    assert!(slice == slice_input);
}
