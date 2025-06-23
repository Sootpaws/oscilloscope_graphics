use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
pub use std::f32::consts::PI;

/// Sample rate of the generated audio
pub const SAMPLE_RATE: u32 = 48000;

/// Pure square wave
#[derive(Default)]
pub struct Square {
    time: f32,
}

impl Square {
    pub fn generate(&mut self) -> (f32, f32) {
        self.time += 1.0 / (SAMPLE_RATE as f32);
        let freq = 440.0;
        let phase = (self.time * freq).fract().round();
        let value = phase * 2.0 - 1.0;
        (value, value)
    }
}

fn main() {
    play();
}

fn play() {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No default output device");
    let config = cpal::StreamConfig {
        channels: 2,
        sample_rate: cpal::SampleRate(SAMPLE_RATE as u32),
        buffer_size: cpal::BufferSize::Default,
    };
    let mut generator = Square::default();
    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut sample_gen = (0.0, 0.0);
                let mut gen_next = true;
                for sample_out in data.iter_mut() {
                    if gen_next {
                        // Left channel
                        sample_gen = generator.generate();
                        *sample_out = sample_gen.0;
                        gen_next = false;
                    } else {
                        // Right channel
                        *sample_out = sample_gen.1;
                        gen_next = true;
                    }
                }
            },
            move |err| {
                eprintln!("Audio output stream error: {err}");
            },
            // No timeout
            None,
        )
        .expect("Failed to create audio output stream");
    stream.play().expect("Failed to play audio output stream");
    std::thread::sleep(std::time::Duration::from_secs_f32(1.0));
}
