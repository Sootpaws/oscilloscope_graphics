use crate::signal::{SAMPLE_RATE, Signal};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

/// Signal output thread
pub struct Player {
    stream: cpal::Stream,
}

impl Player {
    pub fn play(mut signal: Box<dyn Signal>) -> Self {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("No default output device");
        let config = cpal::StreamConfig {
            channels: 2,
            sample_rate: cpal::SampleRate(SAMPLE_RATE),
            buffer_size: cpal::BufferSize::Default,
        };
        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let mut sample_gen = (0.0, 0.0);
                    let mut gen_next = true;
                    for sample_out in data.iter_mut() {
                        if gen_next {
                            // Left channel
                            sample_gen = signal.generate();
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
        Self { stream }
    }
}
