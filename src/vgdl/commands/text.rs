use crate::vgdl::{Command, CommandObj, Lines, State};
use anyhow::{Result, anyhow};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Text;

impl CommandObj for Text {
    fn run(&self, state: &mut State, args: &mut VecDeque<&str>) -> Result<Lines> {
        let mut out = Vec::new();
        let mut offset = 0.5;
        loop {
            let next = args.pop_front().ok_or(anyhow!("Expected string or ."))?;
            if next == "." {
                break Ok(out);
            }
            for char in next.chars() {
                let char_string = char.to_string();
                let mut command = VecDeque::from([char_string.as_str()]);
                let mut glyph = state
                    .exec(&mut command)?
                    .into_iter()
                    .map(|line| line.into_iter().map(|(x, y)| (x + offset, y)).collect())
                    .collect();
                out.append(&mut glyph);
                offset += 2.0;
            }
            offset += 2.0;
        }
    }

    fn dup(&self) -> Command {
        Box::new(self.clone())
    }
}
