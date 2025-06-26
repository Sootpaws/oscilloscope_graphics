use anyhow::Result;
use oscilli_disp::linedraw::Drawer;
use oscilli_disp::signal::player::Player;
use oscilli_disp::vgdl::State;

fn main() -> Result<()> {
    let mut state = State::new();
    let mut player = Player::new()?;
    state.run("load data")?;
    player.play(Drawer::new(state.run("calibration")?));
    std::thread::sleep(std::time::Duration::from_secs_f32(100.0));
    Ok(())
}
