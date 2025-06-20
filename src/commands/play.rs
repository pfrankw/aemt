use std::sync::mpsc::{Receiver, sync_channel};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::{
    args::{Args, PlayArgs},
    kidz::Kidz,
    utils::audio::{decode_adpcm, split_audio_pack, AdpcmState},
};

fn build_stream(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    pcm_rx: Receiver<i16>,
) -> Result<cpal::Stream, cpal::BuildStreamError> {
    let stream = device.build_output_stream(
        config,
        move |output: &mut [i16], _: &cpal::OutputCallbackInfo| {
            for sample in output.iter_mut() {
                match pcm_rx.recv() {
                    Ok(pcm) => {
                        *sample = pcm;
                    }
                    Err(_e) => {
                        *sample = 0;
                    }
                }
            }
        },
        |err| eprintln!("Audio stream error: {}", err),
        None,
    )?;

    Ok(stream)
}

pub fn play(args: &Args, eargs: &PlayArgs) -> Result<(), crate::error::Error> {
    let kidz = Kidz::load(&args.directory)?;

    let file = kidz
        .files
        .get(eargs.index)
        .ok_or(crate::error::Error::Oob)?;

    let host = cpal::default_host();
    let device = host.default_output_device().ok_or("No device")?;

    if let Some(true) = eargs.show_configs {
        let configs: Vec<cpal::SupportedStreamConfigRange> = device.supported_output_configs().map_err(|e| e.to_string())?.collect();
        for config in configs.iter() {
            println!("{:?}", config);
        }
    }

    let sample_rate = 18000;
    let channels = 1;
    let config = cpal::StreamConfig {
        channels: channels as cpal::ChannelCount,
        sample_rate: cpal::SampleRate(sample_rate),
        buffer_size: cpal::BufferSize::Default, // Or specify exact buffer size
    };

    let audios = split_audio_pack(&file.data);

    if audios.is_empty() {
        return Err(crate::error::Error::Generic(
            "No tracks available in the selected file".to_string(),
        ));
    }

    if let Some(track) = eargs.track {
        let audiotrack = audios.get(track).ok_or(crate::error::Error::Oob)?;

        let (tx, rx) = sync_channel(1);
        let stream = build_stream(&device, &config, rx).unwrap();

        stream.play().unwrap();

        let mut state = AdpcmState {
            adpcm_history1_32: 0,
            adpcm_history2_32: 0,
        };

        for chunk in audiotrack.chunks_exact(16) {
            let mut samples = [0i16; 28];

            decode_adpcm(
                &mut state,
                chunk.try_into().unwrap(),
                &mut samples,
                1,
                false,
                0,
            )?;

            for sample in samples {
                tx.send(sample).map_err(|e| e.to_string())?;
            }
        }

        // Dropping stream explicitly for clarity.
        // Dropping tx explicitly because of stream's destructor.
        // Once stream destructor gets called, it somehow blocks because it's waiting for data from
        // rx. Manually dropping tx before stream's destructor gets called frees from this lock.
        std::mem::drop(tx);
        std::mem::drop(stream);
    } else {
        println!(
            "There are {} tracks available in the current file",
            audios.len()
        );
    }

    Ok(())
}
