use oscilli_disp::signal::player::Player;
use oscilli_disp::linedraw::Drawer;
use oscilli_disp::vgdl::State;
use anyhow::Result;

fn main() -> Result<()> {
    let calibration = "draw
        0.0 0.1  0.0 0.0  0.1 0.0 ,
        0.9 0.0  1.0 0.0  1.0 0.1 ,
        0.0 0.9  0.0 1.0  0.1 1.0 ,
        1.0 0.9  1.0 1.0  0.9 1.0 ,
        0.4 0.5  0.6 0.5 ,
        0.5 0.4  0.5 0.6 ,
        0.75 0.55  0.85 0.45 , 0.75 0.45  0.85 0.55 ,
        0.45 0.75  0.5 0.8  0.55 0.75 , 0.5 0.8  0.5 0.85 ;
    ";

    let mut state = State::new();
    let mut player = Player::new()?;
    player.play(Drawer::new(state.run(bk)?));
    std::thread::sleep(std::time::Duration::from_secs_f32(100.0));
    Ok(())
}
