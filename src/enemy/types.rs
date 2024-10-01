use crate::enemy::Trail;
use crate::world::InGameEntity;
use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

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
    Shooter {
        shoot_timer: Timer,
        bullets_per_shot: usize,
        reload_timer: Timer,
        in_range: bool,
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
    pub xp: f32,
}

impl EnemyType {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => EnemyType::Shooter {
                shoot_timer: Timer::from_seconds(0.5, TimerMode::Once),
                bullets_per_shot: rng.gen_range(3..=6),
                reload_timer: Timer::from_seconds(2.0, TimerMode::Once),
                in_range: false,
            },
            1 => EnemyType::LeaveTrail {
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                trail_damage: 4.0,
            },
            2 => EnemyType::Charge {
                state: ChargeState::Approaching,
                charge_timer: Timer::from_seconds(0.5, TimerMode::Once),
                charge_distance: 200.0,
                charge_speed: 15.0,
                target_position: None,
            },
            _ => EnemyType::Basic,
        }
    }

    pub fn get_config(&self) -> EnemyConfig {
        match self {
            EnemyType::Basic => EnemyConfig {
                health: 100.0,
                speed: 6.0,
                damage: 6.0,
                sprite_index: 8,
                xp: 4.0,
            },
            EnemyType::LeaveTrail { .. } => EnemyConfig {
                health: 50.0,
                speed: 10.0,
                damage: 6.0,
                sprite_index: 12,
                xp: 5.0,
            },
            EnemyType::Charge { .. } => EnemyConfig {
                health: 80.0,
                speed: 6.0,
                damage: 8.0,
                sprite_index: 20,
                xp: 8.0,
            },
            EnemyType::Shooter { .. } => EnemyConfig {
                health: 100.0,
                speed: 8.0,
                damage: 0.0,
                sprite_index: 28,
                xp: 10.0,
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
            EnemyType::Shooter { in_range, .. } => {
                let distance = current_pos.distance(player_pos);
                *in_range = distance <= 300.0;
                if *in_range {
                    Vec3::ZERO
                } else {
                    (player_pos - current_pos).normalize() * base_speed
                }
            }
        }
    }

    pub fn apply(&mut self, commands: &mut Commands, transform: &Transform, delta: Duration) {
        if let EnemyType::LeaveTrail {
            timer,
            trail_damage,
        } = self
        {
            timer.tick(delta);
            if timer.just_finished() {
                spawn_trail(commands, transform.translation, *trail_damage);
            }
        }
    }

    pub fn update_shooter(&mut self, delta: Duration) -> Option<usize> {
        if let EnemyType::Shooter {
            shoot_timer,
            bullets_per_shot,
            reload_timer,
            in_range,
        } = self
        {
            if *in_range {
                shoot_timer.tick(delta);
                if shoot_timer.just_finished() {
                    *shoot_timer = Timer::from_seconds(0.1, TimerMode::Once);
                    return Some(*bullets_per_shot);
                }
            } else {
                reload_timer.tick(delta);
                if reload_timer.finished() {
                    *reload_timer = Timer::from_seconds(2.0, TimerMode::Once);
                }
            }
        }
        None
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
