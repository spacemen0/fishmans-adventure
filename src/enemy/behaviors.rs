use super::components::Trail;
use crate::world::InGameEntity;
use bevy::prelude::*;
use std::time::Duration;

pub trait EnemyBehavior: Send + Sync {
    fn update_movement(
        &mut self,
        current_pos: Vec3,
        player_pos: Vec3,
        base_speed: f32,
        delta: Duration,
    ) -> Vec3;
    fn apply(&mut self, commands: &mut Commands, transform: &Transform, delta: Duration);
}

pub struct BasicBehavior;

pub struct LeaveTrailBehavior {
    pub timer: Timer,
    pub trail_damage: f32,
}

pub struct ChargeBehavior {
    pub state: ChargeState,
    pub charge_timer: Timer,
    pub charge_distance: f32,
    pub charge_speed: f32,
    pub target_position: Option<Vec2>,
}

#[derive(Clone, Debug)]
pub enum ChargeState {
    Approaching,
    Preparing,
    Charging,
    Cooldown,
}

impl EnemyBehavior for BasicBehavior {
    fn update_movement(
        &mut self,
        current_pos: Vec3,
        player_pos: Vec3,
        base_speed: f32,
        _delta: Duration,
    ) -> Vec3 {
        (player_pos - current_pos).normalize() * base_speed
    }

    fn apply(&mut self, _commands: &mut Commands, _transform: &Transform, _delta: Duration) {}
}

impl EnemyBehavior for LeaveTrailBehavior {
    fn update_movement(
        &mut self,
        current_pos: Vec3,
        player_pos: Vec3,
        base_speed: f32,
        _delta: Duration,
    ) -> Vec3 {
        (player_pos - current_pos).normalize() * base_speed
    }

    fn apply(&mut self, commands: &mut Commands, transform: &Transform, delta: Duration) {
        self.timer.tick(delta);
        if self.timer.just_finished() {
            spawn_trail(commands, transform.translation, self.trail_damage);
        }
    }
}

impl EnemyBehavior for ChargeBehavior {
    fn update_movement(
        &mut self,
        current_pos: Vec3,
        player_pos: Vec3,
        base_speed: f32,
        delta: Duration,
    ) -> Vec3 {
        self.charge_timer.tick(delta);

        match self.state {
            ChargeState::Approaching => {
                if current_pos.distance(player_pos) <= self.charge_distance {
                    self.state = ChargeState::Preparing;
                    self.charge_timer = Timer::from_seconds(1.5, TimerMode::Once);
                    Vec3::ZERO
                } else {
                    (player_pos - current_pos).normalize() * base_speed
                }
            }
            ChargeState::Preparing => {
                if self.charge_timer.just_finished() {
                    self.state = ChargeState::Charging;
                    self.charge_timer = Timer::from_seconds(2.0, TimerMode::Once);
                    self.target_position = Some(player_pos.truncate());
                }
                Vec3::ZERO
            }
            ChargeState::Charging => {
                if self.charge_timer.just_finished() {
                    self.state = ChargeState::Cooldown;
                    self.charge_timer = Timer::from_seconds(1.5, TimerMode::Once);
                    self.target_position = None;
                    Vec3::ZERO
                } else if let Some(target) = self.target_position {
                    (target.extend(0.0) - current_pos).normalize() * self.charge_speed
                } else {
                    Vec3::ZERO
                }
            }
            ChargeState::Cooldown => {
                if self.charge_timer.just_finished() {
                    self.state = ChargeState::Approaching;
                }
                Vec3::ZERO
            }
        }
    }

    fn apply(&mut self, _commands: &mut Commands, _transform: &Transform, _delta: Duration) {}
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
