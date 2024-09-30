use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;
use crate::enemy::Trail;
use crate::world::InGameEntity;

#[derive(Component, Clone)]
pub enum EnemyType {
    Basic,
    LeaveTrail {
        timer: Timer,
        trail_damage: f32,
    },
    Charge {
        state: ChargeState,
        charge_timer: Timer,
        charge_distance: f32,
        charge_speed: f32,
        target_position: Option<Vec2>,
    },
}

#[derive(Clone, Debug)]
pub enum ChargeState {
    Approaching,
    Preparing,
    Charging,
    Cooldown,
}

pub struct EnemyConfig {
    pub health: f32,
    pub speed: f32,
    pub damage: f32,
    pub sprite_index: usize,
}

impl EnemyType {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => EnemyType::Basic,
            1 => EnemyType::LeaveTrail {
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                trail_damage: 4.0,
            },
            _ => EnemyType::Charge {
                state: ChargeState::Approaching,
                charge_timer: Timer::from_seconds(0.5, TimerMode::Once),
                charge_distance: 200.0,
                charge_speed: 15.0,
                target_position: None,
            },
        }
    }

    pub fn get_config(&self) -> EnemyConfig {
        match self {
            EnemyType::Basic => EnemyConfig {
                health: 100.0,
                speed: 6.0,
                damage: 6.0,
                sprite_index: 8,
            },
            EnemyType::LeaveTrail { .. } => EnemyConfig {
                health: 50.0,
                speed: 10.0,
                damage: 6.0,
                sprite_index: 12,
            },
            EnemyType::Charge { .. } => EnemyConfig {
                health: 80.0,
                speed: 6.0,
                damage: 8.0,
                sprite_index: 20,
            },
        }
    }

    pub fn update_movement(
        &mut self,
        current_pos: Vec3,
        player_pos: Vec3,
        base_speed: f32,
        delta: Duration,
    ) -> Vec3 {
        match self {
            EnemyType::Basic => (player_pos - current_pos).normalize() * base_speed,
            EnemyType::LeaveTrail { .. } => (player_pos - current_pos).normalize() * base_speed,
            EnemyType::Charge {
                state,
                charge_timer,
                charge_distance,
                charge_speed,
                target_position,
            } => {
                charge_timer.tick(delta);

                match state {
                    ChargeState::Approaching => {
                        if current_pos.distance(player_pos) <= *charge_distance {
                            *state = ChargeState::Preparing;
                            *charge_timer = Timer::from_seconds(1.5, TimerMode::Once);
                            Vec3::ZERO
                        } else {
                            (player_pos - current_pos).normalize() * base_speed
                        }
                    }
                    ChargeState::Preparing => {
                        if charge_timer.just_finished() {
                            *state = ChargeState::Charging;
                            *charge_timer = Timer::from_seconds(2.0, TimerMode::Once);
                            *target_position = Some(player_pos.truncate());
                        }
                        Vec3::ZERO
                    }
                    ChargeState::Charging => {
                        if charge_timer.just_finished() {
                            *state = ChargeState::Cooldown;
                            *charge_timer = Timer::from_seconds(1.5, TimerMode::Once);
                            *target_position = None;
                            Vec3::ZERO
                        } else if let Some(target) = target_position {
                            (target.extend(0.0) - current_pos).normalize() * *charge_speed
                        } else {
                            Vec3::ZERO
                        }
                    }
                    ChargeState::Cooldown => {
                        if charge_timer.just_finished() {
                            *state = ChargeState::Approaching;
                        }
                        Vec3::ZERO
                    }
                }
            }
        }
    }

    pub fn apply(&mut self, commands: &mut Commands, transform: &Transform, delta: Duration) {
        if let EnemyType::LeaveTrail { timer, trail_damage } = self {
            timer.tick(delta);
            if timer.just_finished() {
                spawn_trail(commands, transform.translation, *trail_damage);
            }
        }
    }
}

fn spawn_trail(commands: &mut Commands, position: Vec3, damage: f32) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.0, 0.8, 0.0, 0.5),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        },
        Trail {
            damage,
            radius: 10.0,
        },
        crate::gun::HasLifespan::new(Duration::from_secs_f32(5.0)),
        InGameEntity,
    ));
}