use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub health: u32,
    pub speed: u32,
    pub damage: u32,
    pub xp: u32,
}

#[derive(Component)]
pub struct Collider {
    pub radius: u32,
}

#[derive(Component)]
pub struct EnemyBullet;

#[derive(Component)]
pub struct TrailAbility {
    pub timer: Timer,
    pub damage: u32,
    pub trail_radius: f32,
}

#[derive(Component)]
pub struct ExplosionAbility {
    pub explosion_radius: f32,
    pub explosion_damage: u32,
}

#[derive(Component)]
pub struct ShootingAbility {
    pub shoot_timer: Timer,
    pub reload_timer: Timer,
    pub bullets_per_shot: usize,
    pub range: f32,
    pub in_range: bool,
}

#[derive(Component, Debug)]
pub enum ChargeState {
    Approaching,
    Preparing,
    Charging,
    CoolingDown,
}

#[derive(Component)]
pub struct ChargeAbility {
    pub state: ChargeState,
    pub charge_timer: Timer,
    pub charge_distance: u32,
    pub charge_speed: u32,
    pub target_position: Option<Vec2>,
}

#[derive(Component)]
pub struct Trail {
    pub damage: u32,
    pub radius: f32,
}

#[derive(Component)]
pub struct Explosion {
    pub radius: f32,
    pub damage: u32,
    pub timer: Timer,
}

#[derive(Component, Debug)]
pub enum EnemyState {
    Wandering { direction: Vec2, timer: Timer },
    Pursuing,
    Retreating,
    MaintainingDistance,
}

#[derive(Component)]
pub struct RangedBehavior {
    pub preferred_distance: f32,
    pub tolerance: f32,
}

impl Default for EnemyState {
    fn default() -> Self {
        Self::Wandering {
            direction: Vec2::new(1.0, 0.0),
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}
