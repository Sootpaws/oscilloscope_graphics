pub use std::f32::consts::PI;

/// Signal output
pub mod player;

/// Basic waveforms
pub mod waveforms;

/// Sample rate of all signals
pub const SAMPLE_RATE: u32 = 48000;

/// A playable signal
pub trait Signal: Send {
    fn generate(&mut self) -> (f32, f32);
}
