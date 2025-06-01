use std::{
    fs::File,
    io::{Read, Write},
};

use crate::{
    args::{Args, SwapArgs},
    kidz::Kidz,
};

pub fn swap(args: &Args, eargs: &SwapArgs) -> Result<(), crate::error::Error> {
    let mut kidz = Kidz::load(&args.hed, &args.dat, &args.bns)?;
    //
    // let kfile_a= kidz
    //     .files
    //     .get_mut(eargs.index_a)
    //     .ok_or(crate::error::Error::Oob)?;
    // let mut file = File::open(&eargs.input)?;
    // let file_len = file.metadata()?.len();
    //
    // if file_len != kfile.hed.len as u64 * 2048 {
    //     return Err(crate::error::Error::InvalidLength(
    //         "Cannot patch a file with different length".to_string(),
    //     ));
    // }
    //
    // file.read_exact(&mut kfile.data)?;
    //
    // kidz.store(&args.hed, &args.dat, &args.bns)?;
    //
    println!("Archive patched");

    Ok(())
}
