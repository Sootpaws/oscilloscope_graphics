use crate::signal::{SAMPLE_RATE, Signal, waveforms::Silence};
use anyhow::{Result, anyhow};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc;

/// Signal output thread
pub struct Player {
    stream: cpal::Stream,
    comm: mpsc::Sender<Box<dyn Signal>>,
}

impl Player {
    pub fn new() -> Result<Self> {
        let (comm, recv) = mpsc::channel();
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or(anyhow!("No default output device"))?;
        let config = cpal::StreamConfig {
            channels: 2,
            sample_rate: cpal::SampleRate(SAMPLE_RATE),
            buffer_size: cpal::BufferSize::Default,
        };
        let mut signal: Box<dyn Signal> = Silence::new();
        let stream = device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                if let Ok(new_signal) = recv.try_recv() {
                    signal = new_signal;
                }
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
        )?;
        stream.play()?;
        Ok(Self { stream, comm })
    }

    pub fn play(&mut self, signal: Box<dyn Signal>) {
        self.comm.send(signal).unwrap();
    }
}
