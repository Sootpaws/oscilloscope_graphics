use crate::vgdl::{Command, CommandObj, Lines, State};
use anyhow::{Context, Result, anyhow, bail};
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Clone)]
pub struct Draw;

impl CommandObj for Draw {
    fn run(&self, _state: &mut State, args: &mut VecDeque<&str>) -> Result<Lines> {
        let mut out = Vec::new();
        let mut line = Vec::new();
        loop {
            let next = args.pop_front().ok_or(anyhow!("Expected , ; or point"))?;
            if next == "," || next == ";" {
                if line.len() < 2 {
                    bail!("Lines cannot have less than 2 points");
                }
                out.push(line);
                line = Vec::new();
                if next == ";" {
                    if out.is_empty() {
                        bail!("Must have at least one line to draw");
                    }
                    break Ok(out);
                }
            } else {
                let x = next;
                let y = args.pop_front().ok_or(anyhow!("Expected second point"))?;
                let x = f32::from_str(x).context("Cannot parse x coordinate")?;
                let y = f32::from_str(y).context("Cannot parse y coordinate")?;
                line.push((x, y));
            }
        }
    }

    fn dup(&self) -> Command {
        Box::new(self.clone())
    }
}
