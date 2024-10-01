use super::types::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub health: u32,
    pub speed: u32,
    pub damage: u32,
    pub xp: u32,
    pub enemy_type: EnemyType,
}

#[derive(Component)]
pub struct Trail {
    pub damage: u32,
    pub radius: f32,
}

#[derive(Component)]
pub struct Collider {
    pub radius: u32,
}

#[derive(Component)]
pub struct EnemyBullet;
