use crate::{
    args::{Args, HeditArgs},
    kidz::Kidz,
};

fn parse_hex_u16(s: &str) -> Result<u16, std::num::ParseIntError> {
    if s.starts_with("0x") || s.starts_with("0X") {
        u16::from_str_radix(&s[2..], 16)
    } else {
        s.parse::<u16>()
    }
}

fn parse_hex_u32(s: &str) -> Result<u32, std::num::ParseIntError> {
    if s.starts_with("0x") || s.starts_with("0X") {
        u32::from_str_radix(&s[2..], 16)
    } else {
        s.parse::<u32>()
    }
}

pub fn hedit(args: &Args, eargs: &HeditArgs) -> Result<(), crate::error::Error> {
    let mut kidz = Kidz::load(&args.directory)?;

    let offset = parse_hex_u32(&eargs.offset)?;
    let len = parse_hex_u16(&eargs.length)?;

    kidz.hedit(eargs.index, offset, len)?;

    kidz.store(&args.directory)?;

    println!("Archive patched");

    Ok(())
}
