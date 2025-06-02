use std::{fs::File, io::Write};

use crate::{
    args::{Args, ExtractArgs},
    kidz::Kidz,
};

pub fn extract(args: &Args, eargs: &ExtractArgs) -> Result<(), crate::error::Error> {
    let kidz = Kidz::load(&args.directory)?;

    let mut ofile = File::create(&eargs.output)?;
    let kfile = kidz.files.get(eargs.index).ok_or(crate::error::Error::Oob)?;

    ofile.write_all(&kfile.data)?;

    println!("File exported");

    Ok(())
}
