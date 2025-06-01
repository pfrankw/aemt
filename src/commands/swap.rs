use crate::{
    args::{Args, SwapArgs},
    kidz::Kidz,
};

/// Implemented only for testing the boundaries of the KKIIDDZZ.DAT file.
/// It's useless for modding as the only things it does is swapping two files inside the archive.
/// Swapping seems to be permitted but not for all files. It's like some file offsets are hardcoded
/// somewhere.
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
