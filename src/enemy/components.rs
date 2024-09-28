use super::behaviors::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
    pub damage: f32,
    pub behavior: Box<dyn EnemyBehavior>,
}

#[derive(Component)]
pub struct Trail {
    pub damage: f32,
    pub radius: f32,
}

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
}