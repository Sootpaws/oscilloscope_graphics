use oscilli_disp::signal::{player::Player, waveforms::Square};

fn main() {
    let signal = Square::new(100.0);
    let mut player = Player::new();
    player.play(signal);
    std::thread::sleep(std::time::Duration::from_secs_f32(10.0));
}
