use crate::{
    args::Args,
    kidz::{Kidz, FileType},
};

pub fn list(args: &Args) -> Result<(), crate::error::Error> {
    let kidz = Kidz::load(&args.hed, &args.dat, &args.bns)?;

    println!(
        "{:<5} {:<5} {:<10} {:<10}",
        "No.", "Type", "Offset", "Length"
    );

    for (index, file) in kidz.files.iter().enumerate() {
        if let FileType::Empty = file.t {
            continue;
        }

        println!(
            "{:<5} {:<5} {:<10} {:<10}",
            index,
            file.t,
            format!("{:08X}", file.hed.offset),
            file.hed.len
        );
    }

    Ok(())
}
