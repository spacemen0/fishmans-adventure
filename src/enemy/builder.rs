use super::components::*;
use crate::{
    animation::AnimationTimer, loot::LootPool, resources::GlobalTextureAtlas, utils::InGameEntity,
};
use bevy::prelude::*;

pub struct EnemyBuilder {
    health: u32,
    speed: u32,
    damage: u32,
    xp: u32,
    sprite_index: usize,
    abilities: Vec<Box<dyn FnOnce(&mut Commands, Entity)>>,
    loot_pool: Option<LootPool>,
}

impl Default for EnemyBuilder {
    fn default() -> Self {
        Self {
            health: 100,
            speed: 6,
            damage: 6,
            xp: 4,
            sprite_index: 16,
            abilities: Vec::new(),
            loot_pool: None,
        }
    }
}

impl EnemyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_stats(mut self, health: u32, speed: u32, damage: u32, xp: u32) -> Self {
        self.health = health;
        self.speed = speed;
        self.damage = damage;
        self.xp = xp;
        self
    }

    pub fn with_sprite(mut self, sprite_index: usize) -> Self {
        self.sprite_index = sprite_index;
        self
    }

    pub fn with_loot_pool(mut self, loot_pool: LootPool) -> Self {
        self.loot_pool = Some(loot_pool);
        self
    }

    pub fn with_trail(mut self, damage: u32, interval: f32, radius: f32) -> Self {
        self.abilities.push(Box::new(move |commands, entity| {
            commands.entity(entity).insert(TrailAbility {
                timer: Timer::from_seconds(interval, TimerMode::Repeating),
                damage,
                trail_radius: radius,
            });
        }));
        self
    }

    pub fn with_explosion(mut self, radius: f32, damage: u32) -> Self {
        self.abilities.push(Box::new(move |commands, entity| {
            commands.entity(entity).insert(ExplosionAbility {
                explosion_radius: radius,
                explosion_damage: damage,
            });
        }));
        self
    }

    pub fn with_shooting(
        mut self,
        bullets: usize,
        shoot_interval: f32,
        reload_time: f32,
        range: f32,
    ) -> Self {
        self.abilities.push(Box::new(move |commands, entity| {
            commands.entity(entity).insert(ShootingAbility {
                shoot_timer: Timer::from_seconds(shoot_interval, TimerMode::Once),
                reload_timer: Timer::from_seconds(reload_time, TimerMode::Once),
                bullets_per_shot: bullets,
                range,
                in_range: false,
            });
        }));
        self
    }

    pub fn with_charge(mut self, distance: u32, speed: u32, prepare_time: f32) -> Self {
        self.abilities.push(Box::new(move |commands, entity| {
            commands.entity(entity).insert(ChargeAbility {
                state: ChargeState::Approaching,
                charge_timer: Timer::from_seconds(prepare_time, TimerMode::Once),
                charge_distance: distance,
                charge_speed: speed,
                target_position: None,
            });
        }));
        self
    }

    pub fn spawn(
        self,
        commands: &mut Commands,
        position: Vec3,
        handle: &GlobalTextureAtlas,
    ) -> Entity {
        let entity = commands
            .spawn((
                SpriteBundle {
                    texture: handle.image.clone().unwrap(),
                    transform: Transform::from_translation(position).with_scale(Vec3::splat(3.0)),
                    ..default()
                },
                TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: self.sprite_index,
                },
                Enemy {
                    health: self.health,
                    speed: self.speed,
                    damage: self.damage,
                    xp: self.xp,
                },
                EnemyState::default(),
                AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
                Collider { radius: 15 },
                InGameEntity,
            ))
            .id();

        if let Some(loot_pool) = self.loot_pool {
            commands.entity(entity).insert(loot_pool);
        }

        for ability_fn in self.abilities {
            ability_fn(commands, entity);
        }

        entity
    }

    pub fn with_ranged_behavior(mut self, preferred_distance: f32, tolerance: f32) -> Self {
        self.abilities.push(Box::new(move |commands, entity| {
            commands.entity(entity).insert(RangedBehavior {
                preferred_distance,
                tolerance,
            });
        }));
        self
    }
}
