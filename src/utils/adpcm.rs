//! All of this is heavily inspired from vgmstream's PSX ADPCM decoder

// PS-ADPCM table, defined as rational numbers (as in the spec)
const PS_ADPCM_COEFS_F: [[f32; 2]; 16] = [
    [0.0, 0.0],           // {   0.0        ,   0.0        }
    [0.9375, 0.0],        // {  60.0 / 64.0 ,   0.0        }
    [1.796875, -0.8125],  // { 115.0 / 64.0 , -52.0 / 64.0 }
    [1.53125, -0.859375], // {  98.0 / 64.0 , -55.0 / 64.0 }
    [1.90625, -0.9375],   // { 122.0 / 64.0 , -60.0 / 64.0 }
    // Extended table used in few PS3 games, found in ELFs
    [0.46875, -0.0],          // {  30.0 / 64.0 ,  -0.0 / 64.0 }
    [0.8984375, -0.40625],    // {  57.5 / 64.0 , -26.0 / 64.0 }
    [0.765625, -0.4296875],   // {  49.0 / 64.0 , -27.5 / 64.0 }
    [0.953125, -0.46875],     // {  61.0 / 64.0 , -30.0 / 64.0 }
    [0.234375, -0.0],         // {  15.0 / 64.0 ,  -0.0 / 64.0 }
    [0.44921875, -0.203125],  // {  28.75/ 64.0 , -13.0 / 64.0 }
    [0.3828125, -0.21484375], // {  24.5 / 64.0 , -13.75/ 64.0 }
    [0.4765625, -0.234375],   // {  30.5 / 64.0 , -15.0 / 64.0 }
    [0.5, -0.9375],           // {  32.0 / 64.0 , -60.0 / 64.0 }
    [0.234375, -0.9375],      // {  15.0 / 64.0 , -60.0 / 64.0 }
    [0.109375, -0.9375],      // {   7.0 / 64.0 , -60.0 / 64.0 }
];

pub struct AdpcmState {
    pub adpcm_history1_32: i32,
    pub adpcm_history2_32: i32,
}

fn clamp16(sample: i32) -> i16 {
    sample.clamp(i16::MIN as i32, i16::MAX as i32) as i16
}

fn get_low_nibble_signed(byte: u8) -> i32 {
    let nibble = byte & 0x0f;
    if nibble > 7 {
        (nibble as i32) - 16
    } else {
        nibble as i32
    }
}

fn get_high_nibble_signed(byte: u8) -> i32 {
    let nibble = (byte >> 4) & 0x0f;
    if nibble > 7 {
        (nibble as i32) - 16
    } else {
        nibble as i32
    }
}

pub fn decode_adpcm(
    state: &mut AdpcmState,
    frame: &[u8; 16],
    pcm: &mut [i16; 28],
    channel_spacing: usize,
    is_badflags: bool,
    config: i32,
) -> Result<(), crate::error::Error> {
    let extended_mode = config == 1;

    let mut coef_index = ((frame[0] >> 4) & 0xf) as usize;
    let mut shift_factor = (frame[0] & 0xf) as u32;
    let flag = if is_badflags { 0 } else { frame[1] };

    // Upper filters only used in few PS3 games, normally 0
    if !extended_mode {
        if coef_index > 5 {
            eprintln!("PS-ADPCM: incorrect coef_index {}", coef_index);
            coef_index = 0;
        }
        if shift_factor > 12 {
            eprintln!("PS-ADPCM: incorrect shift_factor {}", shift_factor);
            shift_factor = 9; // supposedly, from Nocash PSX docs
        }
    }

    if flag > 7 {
        eprintln!("PS-ADPCM: unknown flag {}", flag);
    }

    let shift_factor = 20_u32.saturating_sub(shift_factor);
    let mut hist1 = state.adpcm_history1_32;
    let mut hist2 = state.adpcm_history2_32;
    let mut sample_count = 0;

    // Decode nibbles
    for i in 0..28 {
        let mut sample = 0i32;

        if flag < 0x07 {
            // with flag 0x07 decoded sample must be 0
            let nibbles = frame[0x02 + (i / 2) as usize];

            let nibble_sample = if (i & 1) != 0 {
                // high nibble (low nibble first)
                get_high_nibble_signed(nibbles)
            } else {
                get_low_nibble_signed(nibbles)
            };

            sample = nibble_sample << shift_factor;
            sample += ((PS_ADPCM_COEFS_F[coef_index][0] * hist1 as f32
                + PS_ADPCM_COEFS_F[coef_index][1] * hist2 as f32)
                * 256.0) as i32;
            sample >>= 8;
        }

        pcm[sample_count] = clamp16(sample);
        sample_count += channel_spacing;

        hist2 = hist1;
        hist1 = sample;
    }

    state.adpcm_history1_32 = hist1;
    state.adpcm_history2_32 = hist2;

    Ok(())
}
