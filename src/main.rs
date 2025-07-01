use anyhow::Result;
use oscilli_disp::linedraw::Drawer;
use oscilli_disp::signal::player::Player;
use oscilli_disp::vgdl::State;

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
