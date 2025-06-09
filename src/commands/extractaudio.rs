use std::fs;

use crate::{
    args::{Args, ExtractAudioArgs},
    kidz::Kidz,
    utils::audio::split_audio_pack,
};

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
