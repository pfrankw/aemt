use std::{fs::File, io::Write};

use crate::{
    args::{Args, ExtractArgs},
    kidz::{Kidz, KidzFileType},
};

pub fn extract(args: &Args, eargs: &ExtractArgs) -> Result<(), crate::error::Error> {
    let kidz = Kidz::load(&args.hed, &args.dat, &args.bns)?;

    let mut ofile = File::create(&eargs.output)?;
    let kfile = kidz.files.get(eargs.index).ok_or("Index out of bound")?;

    ofile.write_all(&kfile.data)?;

    println!("File exported");

    Ok(())
}
