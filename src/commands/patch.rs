use crate::{
    args::{Args, PatchArgs},
    kidz::Kidz,
};

pub fn patch(args: &Args, eargs: &PatchArgs) -> Result<(), crate::error::Error> {
    let mut kidz = Kidz::load(&args.directory)?;

    kidz.patch(eargs.index, std::fs::read(&eargs.input)?)?;

    kidz.store(&args.directory)?;

    println!("Archive patched");

    Ok(())
}
