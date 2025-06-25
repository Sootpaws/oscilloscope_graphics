use oscilli_disp::signal::player::Player;
use oscilli_disp::linedraw::Drawer;
use anyhow::Result;

fn main() -> Result<()> {
    let calibration = vec![
        // Corners
        vec![(0.0, 0.1), (0.0, 0.0), (0.1, 0.0)],
        vec![(0.9, 0.0), (1.0, 0.0), (1.0, 0.1)],
        vec![(0.0, 0.9), (0.0, 1.0), (0.1, 1.0)],
        vec![(1.0, 0.9), (1.0, 1.0), (0.9, 1.0)],
        // Center
        vec![(0.4, 0.5), (0.6, 0.5)],
        vec![(0.5, 0.4), (0.5, 0.6)],
        // Axis
        vec![(0.75, 0.55), (0.85, 0.45)], vec![(0.75, 0.45), (0.85, 0.55)],
        vec![(0.45, 0.75), (0.5, 0.8), (0.55, 0.75)], vec![(0.5, 0.8), (0.5, 0.85)]
    ];
    let signal = Drawer::new(calibration);
    let mut player = Player::new()?;
    player.play(signal);
    std::thread::sleep(std::time::Duration::from_secs_f32(100.0));
    Ok(())
}
