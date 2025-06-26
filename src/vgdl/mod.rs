use std::collections::{HashMap, VecDeque};
use anyhow::{Result, anyhow, bail, Context};

/// The high-level state of a VGDL environment
pub struct State {
    env: HashMap<String, Command>,
}

/// A VGDL command
pub trait CommandObj {
    fn run(&self, state: &mut State, args: &mut VecDeque<&str>) -> Result<Lines>;

    fn dup(&self) -> Command;
}
pub type Command = Box<dyn CommandObj>;

/// A drawable series of lines
pub type Lines = Vec<Vec<(f32, f32)>>;

/// Built-in commands
mod commands;

impl State {
    pub fn new() -> Self {
        let mut env: HashMap<String, Command> = HashMap::new();
        env.insert("draw".to_owned(), Box::new(commands::draw::Draw));
        env.insert("define".to_owned(), Box::new(commands::define::Define));
        env.insert("sequence".to_owned(), Box::new(commands::sequence::Sequence));
        env.insert("load".to_owned(), Box::new(commands::load::Load));
        env.insert("scale".to_owned(), Box::new(commands::scale::Scale));
        Self { env }
    }

    pub fn run(&mut self, program: &str) -> Result<Lines> {
        let mut args = program.split_ascii_whitespace().collect::<VecDeque<_>>();
        let out = self.exec(&mut args)?;
        if let Some(first) = args.front() {
            bail!("Extra words after running command: {}", first);
        }
        Ok(out)
    }

    fn exec(&mut self, args: &mut VecDeque<&str>) -> Result<Lines> {
        let name = args.pop_front()
            .ok_or(anyhow!("No command to run"))?;
        let command: Command = self.env.get(name)
            .ok_or(anyhow!("Command '{}' not found", name))?
            .dup();
        command.run(self, args)
            .context(format!("In command {}", name))
    }
}
