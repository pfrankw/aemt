use crate::{
    args::{Args, ListArgs},
    kidz::{FileType, Kidz},
};

pub fn list(args: &Args, eargs: &ListArgs) -> Result<(), crate::error::Error> {
    let kidz = Kidz::load(&args.hed, &args.dat, &args.bns)?;
    let dat_len = kidz.get_archive_len(FileType::Dat, 0);

    println!(
        "{:<5} {:<5} {:<10} {:<10}",
        "No.", "Type", "Offset", "Length"
    );

    for (index, file) in kidz.files.iter().enumerate() {
        if let FileType::Empty = file.t {
            continue;
        }

        let print_offset = if !eargs.true_bns && file.t == FileType::Bns {
            file.hed.offset - (dat_len as u32 / 2048)
        } else {
            file.hed.offset
        };

        let fmt_offset = if eargs.decimal {
            format!("{}", print_offset)
        } else {
            format!("{:08X}", print_offset)
        };

        let fmt_len = if eargs.decimal {
            format!("{}", file.hed.len)
        } else {
            format!("{:08X}", file.hed.len)
        };

        println!(
            "{:<5} {:<5} {:<10} {:<10}",
            index,
            file.t,
            fmt_offset,
            fmt_len
        );
    }

    Ok(())
}
