use crate::signal::{SAMPLE_RATE, Signal};

/// Pure square wave
pub struct Square {
    frequency: f32,
    time: f32,
}

impl Square {
    pub fn new(frequency: f32) -> Box<Self> {
        Box::new(Square {
            frequency,
            time: 0.0,
        })
    }
}

impl Signal for Square {
    fn generate(&mut self) -> (f32, f32) {
        self.time += 1.0 / (SAMPLE_RATE as f32);
        let phase = (self.time * self.frequency).fract().round();
        let value = phase * 2.0 - 1.0;
        (value, value)
    }
}

/// Always zero
pub struct Silence;

impl Silence {
    pub fn new() -> Box<Self> {
        Box::new(Self)
    }
}

impl Signal for Silence {
    fn generate(&mut self) -> (f32, f32) {
        (0.0, 0.0)
    }
}
