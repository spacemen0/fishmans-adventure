use super::components::*;
use crate::{loot::LootPool, resources::GlobalTextureAtlas};
use bevy::prelude::*;

pub struct EnemyBuilder {
    pub health: u32,
    pub speed: u32,
    pub damage: u32,
    pub xp: u32,
    pub sprite_index: usize,
    pub sprite_size: (u32, u32),
    pub abilities: Vec<Box<dyn FnOnce(&mut Commands, Entity)>>,
    pub loot_pool: Option<LootPool>,
}

impl Default for EnemyBuilder {
    fn default() -> Self {
        Self {
            health: 100,
            speed: 6,
            damage: 6,
            xp: 4,
            sprite_index: 16,
            sprite_size: (16, 16),
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

    pub fn with_sprite(mut self, sprite_index: usize, sprite_size: (u32, u32)) -> Self {
        self.sprite_index = sprite_index;
        self.sprite_size = sprite_size;
        self
    }

    pub fn with_loot_pool(mut self, loot_pool: LootPool) -> Self {
        self.loot_pool = Some(loot_pool);
        self
    }

    pub fn with_trail(mut self, damage: u32, interval: f32, radius: f32, duration: f32) -> Self {
        self.abilities.push(Box::new(move |commands, entity| {
            commands.entity(entity).insert(TrailAbility {
                timer: Timer::from_seconds(interval, TimerMode::Repeating),
                damage,
                trail_radius: radius,
                trail_duration: duration,
                last_position: None,
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
        range: f32,
        bullet_speed: u32,
        bullet_damage: u32,
    ) -> Self {
        self.abilities.push(Box::new(move |commands, entity| {
            commands.entity(entity).insert(ShootingAbility {
                shoot_timer: Timer::from_seconds(shoot_interval, TimerMode::Repeating),
                bullets_per_shot: bullets,
                range,
                in_range: false,
                bullet_speed,
                bullet_damage,
            });
        }));
        self
    }

    pub fn with_charge(mut self, distance: u32, speed: u32, prepare_time: f32, cooldown: f32) -> Self {
        self.abilities.push(Box::new(move |commands, entity| {
            commands.entity(entity).insert(ChargeAbility {
                state: ChargeState::Approaching,
                charge_timer: Timer::from_seconds(prepare_time, TimerMode::Once),
                charge_distance: distance,
                charge_speed: speed,
                target_position: None,
                cooldown_duration: cooldown,
            });
        }));
        self
    }

    pub fn with_splitting(mut self, splits: u8) -> Self {
        self.abilities.push(Box::new(move |commands, entity| {
            commands.entity(entity).insert(SplitAbility {
                splits_remaining: splits,
                num_splits: splits,
            });
        }));
        self
    }

    pub fn with_summoning(mut self, min_minions: u32, max_minions: u32, interval: f32) -> Self {
        self.abilities.push(Box::new(move |commands, entity| {
            commands.entity(entity).insert(SummoningAbility {
                timer: Timer::from_seconds(interval, TimerMode::Repeating),
                min_minions,
                max_minions,
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
        let layout = match self.sprite_size {
            (16, 16) => handle.layout_16x16.clone().unwrap(),
            (32, 32) => handle.layout_32x32.clone().unwrap(),
            _ => handle.layout_16x16.clone().unwrap(),
        };
        let entity = commands
            .spawn((
                Name::new("Enemy"),
                Sprite {
                    image: handle.image.clone().unwrap(),
                    texture_atlas: Some(TextureAtlas {
                        layout,
                        index: self.sprite_index,
                    }),
                    ..default()
                },
                Transform::from_translation(position).with_scale(Vec3::splat(3.0)),
                Enemy {
                    health: self.health,
                    speed: self.speed,
                    damage: self.damage,
                    xp: self.xp,
                },
                EnemyState::default(),
                Collider { radius: 15 },
                OriginalEnemyColor(Color::WHITE),
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

    pub fn with_gurgle_marker(mut self) -> Self {
        self.abilities.push(Box::new(|commands, entity| {
            commands.entity(entity).insert(GurgleEnemy);
        }));
        self
    }

    pub fn with_separation(mut self, radius: f32, force: f32) -> Self {
        self.abilities.push(Box::new(move |commands, entity| {
            commands.entity(entity).insert(SeparationBehavior {
                radius,
                force,
            });
        }));
        self
    }
}
