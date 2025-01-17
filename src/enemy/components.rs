use bevy::prelude::*;

use crate::{
    animation::AnimationTimer,
    configs::SPRITE_SCALE_FACTOR,
    gun::{BulletStats, HasLifespan},
    loot::LootPool,
    utils::InGameEntity,
};

#[derive(Component)]
#[require(Sprite, Transform, AnimationTimer(||AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating))), InGameEntity, LootPool, Collider)]
pub struct Enemy {
    pub health: u32,
    pub speed: u32,
    pub damage: u32,
    pub xp: u32,
}

#[derive(Component, Default)]
pub struct Collider {
    pub radius: u32,
}

#[derive(Component)]
pub struct SplitAbility {
    pub splits_remaining: u8,
    pub num_splits: u8,
}

#[derive(Component)]
#[require(InGameEntity,HasLifespan(||HasLifespan::new(std::time::Duration::from_secs(2))),BulletStats(||            BulletStats {
                speed: 200,
                damage: 10,
                lifespan: 2.0,
            }))]
pub struct EnemyBullet;

#[derive(Component)]
pub struct TrailAbility {
    pub timer: Timer,
    pub damage: u32,
    pub trail_radius: f32,
    pub trail_duration: f32,
    pub last_position: Option<Vec3>,
}

#[derive(Component)]
pub struct TrailSegment {
    pub start: Vec3,
    pub end: Vec3,
    pub timer: Timer,
    pub width: f32,
}

#[derive(Component)]
pub struct ExplosionAbility {
    pub explosion_radius: f32,
    pub explosion_damage: u32,
}

#[derive(Component)]
pub struct ShootingAbility {
    pub shoot_timer: Timer,
    pub bullets_per_shot: usize,
    pub range: f32,
    pub in_range: bool,
    pub bullet_speed: u32,
    pub bullet_damage: u32,
}

#[derive(Component, Debug)]
pub enum ChargeState {
    Approaching,
    Preparing,
    Charging,
    Cooldown,
}

#[derive(Component)]
pub struct ChargeAbility {
    pub state: ChargeState,
    pub charge_timer: Timer,
    pub charge_distance: u32,
    pub charge_speed: u32,
    pub target_position: Option<Vec2>,
    pub cooldown_duration: f32,
}

#[derive(Component)]
pub struct OriginalEnemyColor(pub Color);

#[derive(Component)]
pub struct SpawnIndicator {
    pub timer: Timer,
    pub spawn_position: Vec3,
}

#[derive(Component)]
pub struct HitFlash(pub Timer);

impl Default for HitFlash {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Once))
    }
}

#[derive(Component)]
pub struct DeathEffect {
    pub timer: Timer,
    pub initial_scale: Vec3,
}

impl Default for DeathEffect {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.3, TimerMode::Once),
            initial_scale: Vec3::splat(SPRITE_SCALE_FACTOR),
        }
    }
}

#[derive(Component)]
#[require(InGameEntity, HasLifespan(||HasLifespan::new(std::time::Duration::from_secs_f32(5.0))))]
pub struct Trail {
    pub damage: u32,
    pub radius: f32,
}

#[derive(Component)]
#[require(InGameEntity)]
pub struct Explosion {
    pub radius: f32,
    pub damage: u32,
    pub timer: Timer,
}

#[derive(Component)]
pub struct ExplodingBullet {
    pub radius: f32,
    pub damage: u32,
}

#[derive(Component)]
pub struct GurgleEnemy;

#[derive(Component, Debug)]
pub enum EnemyState {
    Wandering { direction: Vec2, timer: Timer },
    Pursuing,
    MaintainingDistance,
    Retreating,
}

impl Default for EnemyState {
    fn default() -> Self {
        Self::Wandering {
            direction: Vec2::new(1.0, 0.0),
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct RangedBehavior {
    pub preferred_distance: f32,
    pub tolerance: f32,
}

#[derive(Component)]
pub struct SummoningAbility {
    pub timer: Timer,
    pub min_minions: u32,
    pub max_minions: u32,
}

#[derive(Event)]
pub struct BomberExplosionEvent {
    pub translation: Vec3,
    pub explosion_radius: f32,
    pub explosion_damage: u32,
}
