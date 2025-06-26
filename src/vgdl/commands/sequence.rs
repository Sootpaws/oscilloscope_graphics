use crate::vgdl::{Command, CommandObj, Lines, State};
use anyhow::Result;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Sequence;

impl CommandObj for Sequence {
    fn run(&self, state: &mut State, args: &mut VecDeque<&str>) -> Result<Lines> {
        let mut out = Vec::new();
        loop {
            if let Some(&".") = args.front() {
                args.pop_front();
                break Ok(out);
            }
            let mut next = state.exec(args)?;
            out.append(&mut next);
        }
    }

    fn dup(&self) -> Command {
        Box::new(self.clone())
    }
}
