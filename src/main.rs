extern crate hound;

use std::f32::consts::PI;
use std::i16;
use hound::*;

fn write_sine_wav() {
    let sampling_freq = 44100;           // 44.1 kHz, a common sampling frequency
    let sampling_bits = 16;              // 16 bits per sample, a la CDDA
    let amplitude     = i16::MAX as f32; // Make it loud
    let note_freq     = 440.0;           // A440, the A abouve middle C.
    let length        = 2;               // length of the sound in seconds

    let no_of_samples  = sampling_freq * length;
    let normalized_sample_indices = (0 .. no_of_samples).
        map(|x| x as f32 / sampling_freq as f32);


    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sampling_freq,
        bits_per_sample: sampling_bits
    };

    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    for t in normalized_sample_indices {
        let sample = (t * note_freq * 2.0 * PI).sin();
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
}


fn main() {
    write_sine_wav();
}
