use std::{
    fs::File,
    io::{Read, Write},
};

use crate::{
    args::{Args, ExtractArgs, PatchArgs},
    kidz::Kidz,
};

pub fn patch(args: &Args, eargs: &PatchArgs) -> Result<(), crate::error::Error> {
    let mut kidz = Kidz::load(&args.hed, &args.dat, &args.bns)?;

    let kfile = kidz
        .files
        .get_mut(eargs.index)
        .ok_or(crate::error::Error::Oob)?;
    let mut file = File::open(&eargs.input)?;
    let file_len = file.metadata()?.len();

    if file_len != kfile.hed.len as u64 * 2048 {
        return Err(crate::error::Error::InvalidLength(
            "Cannot patch a file with different length".to_string(),
        ));
    }

    file.read_exact(&mut kfile.data)?;

    kidz.store(&args.hed, &args.dat, &args.bns)?;

    println!("Archive patched");

    Ok(())
}
