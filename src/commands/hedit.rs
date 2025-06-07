use crate::{
    args::{Args, HeditArgs},
    kidz::Kidz, utils::hexnum::{parse_hex_u16, parse_hex_u32},
};

pub fn hedit(args: &Args, eargs: &HeditArgs) -> Result<(), crate::error::Error> {
    let mut kidz = Kidz::load(&args.directory)?;

    let offset = parse_hex_u32(&eargs.offset)?;
    let len = parse_hex_u16(&eargs.length)?;

    kidz.hedit(eargs.index, offset, len)?;

    kidz.store(&args.directory)?;

    println!("Archive patched");

    Ok(())
}
