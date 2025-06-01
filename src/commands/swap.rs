use crate::{
    args::{Args, SwapArgs},
    kidz::Kidz,
};

pub fn swap(args: &Args, eargs: &SwapArgs) -> Result<(), crate::error::Error> {
    let mut kidz = Kidz::load(&args.hed, &args.dat, &args.bns)?;

    println!(
        "Swapping data at index {} with data at index {} and vice versa",
        eargs.index_a, eargs.index_b
    );

    kidz.swap(eargs.index_a, eargs.index_b)?;
    kidz.store(&args.hed, &args.dat, &args.bns)?;

    println!("Archive patched");

    Ok(())
}
