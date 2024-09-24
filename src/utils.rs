pub fn calculate_enemies_per_wave(wave_number: u32) -> u32 {
    let base_enemies = 10;
    let increase = (wave_number as f32 * 0.5).floor() as u32 * 3;
    base_enemies + increase
}
