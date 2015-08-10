extern crate hound;
extern crate gnuplot;

use std::f32::consts::PI;
use std::i16;
use hound::*;
use gnuplot::*;

fn draw_plot<K, T, S>(xs: T, ys: S, name: &str)
    where K: DataType,
          T: Iterator<Item=K>,
          S: Iterator<Item=K>
{
    let mut fg = Figure::new();
    fg.set_terminal("png", name);
    fg.axes2d().
        set_title("Sine wave - 44.1kHz", &[]).
        lines(xs, ys, &[Caption("sine")]);
    fg.show();
}

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

    let maybe_writer = hound::WavWriter::create("sine.wav", spec);

    let mut xs: Vec<f32> = Vec::with_capacity(no_of_samples as usize);
    let mut ys: Vec<f32> = Vec::with_capacity(no_of_samples as usize);

    match maybe_writer {
        Ok(writer_obj) => {
            let mut writer = writer_obj;
            for t in normalized_sample_indices {
                let sample = (t * note_freq * 2.0 * PI).sin();
                xs.push(t);
                ys.push(sample);
                writer.write_sample((sample * amplitude) as i16).unwrap();
            }
        },
        Err(e) => {
            println!("Oh Noes!");
            println!("{}", e);
        }
    }

    draw_plot(xs.iter(), ys.iter(), "sine.png");
    draw_plot(xs.iter().take(110), ys.iter().take(110), "sine2.png");
}

fn main() {
    write_sine_wav();
}
