use anyhow::Result;
use oscilloscope_graphics::linedraw::Drawer;
use oscilloscope_graphics::signal::player::Player;
use oscilloscope_graphics::vgdl::State;

fn main() -> Result<()> {
    let mut state = State::new();
    let mut player = Player::new()?;
    loop {
        match state.run("load drawing.vgdl") {
            Ok(lines) => {
                if !lines.is_empty() {
                    player.play(Drawer::new(lines));
                }
            }
            Err(msg) => println!("Error: {:#}", msg),
        }
        std::thread::sleep(std::time::Duration::from_secs_f32(10.0));
    }
}
