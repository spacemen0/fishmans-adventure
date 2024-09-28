use super::behaviors::*;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Clone)]
pub enum EnemyType {
    Basic,
    LeaveTrail,
    Charge,
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
            1 => EnemyType::LeaveTrail,
            _ => EnemyType::Charge,
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
            EnemyType::LeaveTrail => EnemyConfig {
                health: 50.0,
                speed: 10.0,
                damage: 6.0,
                sprite_index: 12,
            },
            EnemyType::Charge => EnemyConfig {
                health: 80.0,
                speed: 6.0,
                damage: 8.0,
                sprite_index: 20,
            },
        }
    }

    pub fn get_behavior(&self) -> Box<dyn EnemyBehavior> {
        match self {
            EnemyType::Basic => Box::new(BasicBehavior),
            EnemyType::LeaveTrail => Box::new(LeaveTrailBehavior {
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                trail_damage: 4.0,
            }),
            EnemyType::Charge => Box::new(ChargeBehavior {
                state: ChargeState::Approaching,
                charge_timer: Timer::from_seconds(0.5, TimerMode::Once),
                charge_distance: 200.0,
                charge_speed: 15.0,
                target_position: None,
            }),
        }
    }
}
