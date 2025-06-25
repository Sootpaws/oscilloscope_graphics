use std::collections::{HashMap, VecDeque};

/// The high-level state of a VGDL environment
pub struct State {
    env: HashMap<String, Command>,
}

/// A VGDL command
pub type Command = Box<dyn Fn (&mut State, &mut VecDeque<String>) -> Lines>;

/// A drawable series of lines
pub type Lines = Vec<Vec<(f32, f32)>>;
