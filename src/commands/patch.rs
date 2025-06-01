use crate::{
    args::{Args, PatchArgs},
    kidz::Kidz,
};

pub fn patch(args: &Args, eargs: &PatchArgs) -> Result<(), crate::error::Error> {
    let mut kidz = Kidz::load(&args.hed, &args.dat, &args.bns)?;

    kidz.patch(eargs.index, std::fs::read(&eargs.input)?)?;

    kidz.store(&args.hed, &args.dat, &args.bns)?;

    println!("Archive patched");

    Ok(())
}
