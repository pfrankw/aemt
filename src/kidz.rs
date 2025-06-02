use std::{
    array::TryFromSliceError,
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

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

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum FileType {
    Empty,
    Dat,
    Bns,
    Str,
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ret = match self {
            FileType::Str => "STR",
            FileType::Dat => "DAT",
            FileType::Bns => "BNS",
            FileType::Empty => "Empty",
        };

        ret.fmt(f)
    }
}

pub struct KidzFile {
    pub hed: HedEntry,
    pub data: Vec<u8>,
    pub t: FileType,
}

impl KidzFile {}

pub struct Kidz {
    pub files: Vec<KidzFile>,
}

impl Kidz {
    fn read_all_files(
        entries: &[HedEntry],
        dat: &mut File,
        bns: &mut File,
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
                        t: FileType::Empty,
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
                                dat.seek(SeekFrom::Start(position))?;
                                dat.read_exact(&mut data).or(Err("Failed to read DAT"))?;

                                filetype = FileType::Dat;
                            }

                            // If position is over the DAT file it means we need to read from the
                            // BNS file
                            false => {
                                let position = position - dat_len;

                                bns.seek(SeekFrom::Start(position))?;
                                bns.read_exact(&mut data).or(Err("Failed to read BNS"))?;

                                filetype = FileType::Bns;
                            }
                        },
                        // If > 700h we are reading an STR file
                        false => filetype = FileType::Str,
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

    pub fn load<P: AsRef<Path>>(directory: P) -> Result<Self, crate::error::Error> {
        let mut hed = File::open(directory.as_ref().join("KKIIDDZZ.HED"))?;
        let mut dat = File::open(directory.as_ref().join("KKIIDDZZ.DAT"))?;
        let mut bns = File::open(directory.as_ref().join("KKIIDDZZ.BNS"))?;

        let mut buf = [0u8; 0x800];

        hed.read_exact(&mut buf)
            .or(Err("Failed to read the HED file"))?;

        let entries: Result<Vec<HedEntry>, TryFromSliceError> =
            buf.chunks_exact(4).map(HedEntry::try_from).collect();
        let entries = entries?;

        Ok(Self {
            files: Self::read_all_files(&entries, &mut dat, &mut bns)?,
        })
    }

    pub fn get_archive_len(&self, t: FileType, dat_len: usize) -> usize {
        let last_file = self
            .files
            .iter()
            .filter(|file| file.t == t)
            .max_by_key(|x| x.hed.offset)
            .unwrap();

        if t == FileType::Bns {
            (last_file.hed.offset as usize * 2048) + (last_file.hed.len as usize * 2048) - dat_len
        } else {
            (last_file.hed.offset as usize * 2048) + (last_file.hed.len as usize * 2048)
        }
    }

    fn get_file_hed(&self, index: usize) -> Option<HedEntry> {
        self.files.get(index).map(|file| file.hed)
    }

    pub fn _get(&self, index: usize) -> Option<&KidzFile> {
        self.files.get(index)
    }

    pub fn hedit(
        &mut self,
        index: usize,
        offset: u32,
        len: u16,
    ) -> Result<(), crate::error::Error> {
        let file = self.files.get_mut(index).ok_or(crate::error::Error::Oob)?;

        file.hed.offset = offset;
        file.hed.len = len;

        Ok(())
    }

    pub fn swap(&mut self, index_a: usize, index_b: usize) -> Result<(), crate::error::Error> {
        let data_a = self
            .files
            .get(index_a)
            .ok_or(crate::error::Error::Oob)?
            .data
            .clone();
        let data_b = self
            .files
            .get(index_b)
            .ok_or(crate::error::Error::Oob)?
            .data
            .clone();

        self.patch(index_a, data_b)?;
        self.patch(index_b, data_a)?;

        let tmp_hed = self.files[index_b].hed;
        self.files[index_b].hed = self.files[index_a].hed;
        self.files[index_a].hed = tmp_hed;

        Ok(())
    }

    pub fn patch(&mut self, index: usize, data: Vec<u8>) -> Result<(), crate::error::Error> {
        if data.len() % 2048 != 0 {
            return Err(crate::error::Error::InvalidLength(
                "Length must be a multiple of 2048".to_string(),
            ));
        // hed.len is a 12 bit number, so the maximum sector_len must be 4095
        } else if (data.len() / 2048) > ((2usize.pow(12)) - 1) {
            return Err(crate::error::Error::InvalidLength(
                "File is too big. Maximum allowed size is 8386560 bytes".to_string(),
            ));
        }

        let data_sec_len: u16 = (data.len() / 2048) as u16;
        let hed = self.get_file_hed(index).ok_or(crate::error::Error::Oob)?;

        if data_sec_len != hed.len {
            let mut afterfiles: Vec<&mut KidzFile> = self
                .files
                .iter_mut()
                .filter(|f| f.hed.offset > hed.offset)
                .collect();

            if data_sec_len > hed.len {
                afterfiles
                    .iter_mut()
                    .for_each(|f| f.hed.offset += (data_sec_len - hed.len) as u32);
            } else {
                afterfiles
                    .iter_mut()
                    .for_each(|f| f.hed.offset -= (hed.len - data_sec_len) as u32);
            }
        }

        let file = self.files.get_mut(index).ok_or(crate::error::Error::Oob)?;

        file.data = data;
        file.hed.len = data_sec_len;

        Ok(())
    }

    pub fn store<P: AsRef<Path>>(&self, directory: P) -> Result<(), crate::error::Error> {
        let mut hed = File::create(directory.as_ref().join("KKIIDDZZ.HED"))?;
        let mut dat = File::create(directory.as_ref().join("KKIIDDZZ.DAT"))?;
        let mut bns = File::create(directory.as_ref().join("KKIIDDZZ.BNS"))?;

        let dat_len = self.get_archive_len(FileType::Dat, 0);
        let bns_len = self.get_archive_len(FileType::Bns, dat_len);

        dat.set_len(dat_len as u64)?;
        bns.set_len(bns_len as u64)?;

        for file in self.files.iter() {
            let hed_index: u32 = file.hed.into();
            hed.write_all(&hed_index.to_le_bytes())?;

            match file.t {
                FileType::Dat => {
                    dat.seek(SeekFrom::Start(file.hed.offset as u64 * 2048))?;
                    dat.write_all(&file.data)?;
                }
                FileType::Bns => {
                    bns.seek(SeekFrom::Start(
                        (file.hed.offset as u64 * 2048) - dat_len as u64,
                    ))?;
                    bns.write_all(&file.data)?;
                }
                _ => {}
            }
        }

        Ok(())
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
