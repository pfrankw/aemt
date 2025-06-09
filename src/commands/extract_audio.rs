use std::fs;

use crate::{
    args::{Args, ExtractAudioArgs},
    kidz::Kidz,
    utils::audio::split_audio_pack,
};

/// Inside the KKIIDDZZ.DAT there are multiple kind of files. Some of them are files that contain
/// ADPCM, which are called sound packs (in the context of this project at least). For example, all
/// audio related to Spike's actions are located in the file at index 10 (index starts from 0).
/// Inside this file there are multiple sound "tracks" that can be extracted and played using
/// programs like vgmstream etc.
pub fn extract_audio(args: &Args, eargs: &ExtractAudioArgs) -> Result<(), crate::error::Error> {
    let kidz = Kidz::load(&args.directory)?;

    let file = kidz
        .files
        .get(eargs.index)
        .ok_or(crate::error::Error::Oob)?;

    let audios = split_audio_pack(&file.data);

    let track = audios.get(eargs.track).ok_or(crate::error::Error::Oob)?;

    fs::write(&eargs.output, track)?;

    println!("Audio track exported");

    Ok(())
}
