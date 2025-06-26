use crate::vgdl::{State, Lines, CommandObj, Command};
use anyhow::{Result, anyhow};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Define;

impl CommandObj for Define {
    fn run(&self, state: &mut State, args: &mut VecDeque<&str>) -> Result<Lines> {
        let name = args.pop_front()
            .ok_or(anyhow!("Expected name to define"))?;
        let value = state.exec(args)?;
        state.env.insert(name.to_string(), Binding::new(value));
        Ok(Vec::new())
    }

    fn dup(&self) -> Command {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct Binding {
    value: Lines,
}

impl Binding {
    pub fn new(value: Lines) -> Command {
        Box::new(Self { value })
    }
}

impl CommandObj for Binding {
    fn run(&self, _state: &mut State, _args: &mut VecDeque<&str>) -> Result<Lines> {
        Ok(self.value.clone())
    }

    fn dup(&self) -> Command {
        Box::new(self.clone())
    }
}
