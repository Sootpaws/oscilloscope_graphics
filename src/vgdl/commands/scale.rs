use crate::vgdl::{Command, CommandObj, Lines, State};
use anyhow::{Context, Result, anyhow};
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Clone)]
pub struct Scale;

impl CommandObj for Scale {
    fn run(&self, state: &mut State, args: &mut VecDeque<&str>) -> Result<Lines> {
        let xs = args.pop_front().ok_or(anyhow!("Expected X scale"))?;
        let ys = args.pop_front().ok_or(anyhow!("Expected Y scale"))?;
        let xs = f32::from_str(xs).context("Cannot parse X scale")?;
        let ys = f32::from_str(ys).context("Cannot parse Y scale")?;
        Ok(state
            .exec(args)?
            .into_iter()
            .map(|line| line.into_iter().map(|(x, y)| (x * xs, y * ys)).collect())
            .collect())
    }

    fn dup(&self) -> Command {
        Box::new(self.clone())
    }
}
