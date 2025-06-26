use crate::vgdl::{Command, CommandObj, Lines, State};
use anyhow::Result;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Col;

impl CommandObj for Col {
    fn run(&self, state: &mut State, args: &mut VecDeque<&str>) -> Result<Lines> {
        let mut out = Vec::new();
        let mut offset = 0.5;
        loop {
            if let Some(&".") = args.front() {
                args.pop_front();
                break Ok(out);
            }
            let mut next = state
                .exec(args)?
                .into_iter()
                .map(|line| line.into_iter().map(|(x, y)| (x, y + offset)).collect())
                .collect();
            out.append(&mut next);
            offset += 2.0;
        }
    }

    fn dup(&self) -> Command {
        Box::new(self.clone())
    }
}
