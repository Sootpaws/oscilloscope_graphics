use oscilli_disp::signal;

fn main() {
    let player = signal::player::Player::play(signal::waveforms::Square::new(100.0));
    std::thread::sleep(std::time::Duration::from_secs_f32(10.0));
}
