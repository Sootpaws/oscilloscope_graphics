use crate::vgdl::{Command, CommandObj, Lines, State};
use anyhow::{Context, Result, anyhow};
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Clone)]
pub struct Move;

impl CommandObj for Move {
    fn run(&self, state: &mut State, args: &mut VecDeque<&str>) -> Result<Lines> {
        let xo = args.pop_front().ok_or(anyhow!("Expected X translation"))?;
        let yo = args.pop_front().ok_or(anyhow!("Expected Y translation"))?;
        let xo = f32::from_str(xo).context("Cannot parse X translation")?;
        let yo = f32::from_str(yo).context("Cannot parse Y translation")?;
        Ok(state
            .exec(args)?
            .into_iter()
            .map(|line| line.into_iter().map(|(x, y)| (x + xo, y + yo)).collect())
            .collect())
    }

    fn dup(&self) -> Command {
        Box::new(self.clone())
    }
}
