pub fn calculate_enemies_per_wave(wave_number: u32) -> u32 {
    let base_enemies = 10;
    let increase = (wave_number as f32 * 0.5).floor() as u32 * 3;
    base_enemies + increase
}

pub fn calculate_health_increase(level: u32) -> f32 {
    let base_health_increase = 10.0;
    let exponential_factor: f32 = 1.05;
    let diminishing_return_factor: f32 = 0.95;

    base_health_increase
        * (exponential_factor.powi(level as i32))
        * diminishing_return_factor.powi(level as i32)
}

pub fn calculate_defense_increase(level: u32) -> f32 {
    let base_defense_increase = 2.0;
    let exponential_factor: f32 = 1.03;
    let diminishing_return_factor: f32 = 0.97;

    base_defense_increase
        * (exponential_factor.powi(level as i32))
        * diminishing_return_factor.powi(level as i32)
}
