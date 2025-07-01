use crate::vgdl::{Command, CommandObj, Lines, State};
use anyhow::{Context, Result, anyhow};
use std::collections::VecDeque;
use std::fs;
use std::path::Path;

#[derive(Clone)]
pub struct Load;

impl CommandObj for Load {
    fn run(&self, state: &mut State, args: &mut VecDeque<&str>) -> Result<Lines> {
        let path = args.pop_front().ok_or(anyhow!("Expected path"))?;
        load(path, state).context(format!("While loading {}", path))
    }

    fn dup(&self) -> Command {
        Box::new(self.clone())
    }
}

fn load(path: impl AsRef<Path>, state: &mut State) -> Result<Lines> {
    if fs::metadata(&path)?.is_dir() {
        let mut out = Vec::new();
        for entry in fs::read_dir(path)? {
            let mut result = load(entry?.path(), state)?;
            out.append(&mut result);
        }
        Ok(out)
    } else {
        let contents = fs::read_to_string(path)?;
        state.run(&contents)
    }
}
