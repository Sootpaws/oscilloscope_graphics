use crate::signal::Signal;

/// Line-drawing signal
pub struct Drawer {
    lines: Vec<Vec<(f32, f32)>>,
    line_index: usize,
    segment_index: usize,
    beam_x: f32,
    beam_y: f32,
}

impl Drawer {
    pub fn new(lines: Vec<Vec<(f32, f32)>>) -> Box<Self> {
        Box::new(Self {
            lines,
            line_index: 0,
            segment_index: 0,
            beam_x: 0.0,
            beam_y: 0.0,
        })
    }
}

impl Signal for Drawer {
    fn generate(&mut self) -> (f32, f32) {
        // Get point to move towards
        let line = &self.lines[self.line_index];
        let target = line[self.segment_index];

        // Raw delta
        let delta_x = target.0 - self.beam_x;
        let delta_y = target.1 - self.beam_y;
        // Normalize
        let delta = (delta_x * delta_x + delta_y * delta_y).sqrt();
        if delta != 0.0 {
            let delta = delta / delta.min(DRAW_RATE);
            let delta_x = delta_x / delta;
            let delta_y = delta_y / delta;
            // Move bean
            self.beam_x = self.beam_x + delta_x;
            self.beam_y = self.beam_y + delta_y;
        }

        // Check for target reached
        if delta_x.abs() < DIST_THRESH && delta_y.abs() < DIST_THRESH {
            // Select next segment
            self.segment_index += 1;
            if self.segment_index >= line.len() {
                self.segment_index = 1;
                self.line_index += 1;
                if self.line_index >= self.lines.len() {
                    self.line_index = 0;
                }
                // Move to start
                let (x, y) = self.lines[self.line_index][0];
                self.beam_x = x;
                self.beam_y = y;
            }
        }

        (self.beam_x, self.beam_y)
    }
}

/// Distance to move the beam each sample while drawing
const DRAW_RATE: f32 = 0.007;

/// Distance to line endpoint at which it is considered "reached"
const DIST_THRESH: f32 = 0.000001;
