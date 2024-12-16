use bevy::prelude::*;
use rand::Rng;

use crate::configs::{WH, WW};

#[derive(Component)]
pub struct InGameEntity;

#[derive(Component)]
pub struct Pickable;
pub fn calculate_enemies_per_wave(_wave_number: u32) -> u32 {
    let base_enemies = 1;
    //let increase = (wave_number as f32 * 0.5).floor() as u32 * 3;
    let increase = 2;
    base_enemies + increase
}

pub fn calculate_health_increase(level: u32) -> u32 {
    let base_health_increase = 10.0;
    let exponential_factor: f32 = 1.05;
    let diminishing_return_factor: f32 = 0.95;

    let health_increase = base_health_increase
        * (exponential_factor.powi(level as i32))
        * diminishing_return_factor.powi(level as i32);

    health_increase.round() as u32
}

pub fn calculate_defense_increase(level: u32) -> u32 {
    let base_defense_increase = 2.0;
    let exponential_factor: f32 = 1.03;
    let diminishing_return_factor: f32 = 0.97;

    let defense_increase = base_defense_increase
        * (exponential_factor.powi(level as i32))
        * diminishing_return_factor.powi(level as i32);

    defense_increase.round() as u32
}

pub fn get_random_position_around(pos: Vec2, dist_range: std::ops::Range<f32>) -> (f32, f32) {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let dist = rng.gen_range(dist_range);

    let offset_x = angle.cos() * dist;
    let offset_y = angle.sin() * dist;

    let random_x = pos.x + offset_x;
    let random_y = pos.y + offset_y;

    (random_x.clamp(-WW, WW), random_y.clamp(-WH, WH))
}

pub fn safe_subtract(a: u32, b: u32) -> u32 {
    a.saturating_sub(b)
}

pub fn clamp_position(position: &mut Vec3) {
    position.x = position.x.clamp(-WW, WW);
    position.y = position.y.clamp(-WH, WH);
}
