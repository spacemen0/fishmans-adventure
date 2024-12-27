use super::builder::EnemyBuilder;
use crate::loot::{medium_enemies_bundle, strong_enemies_bundle, weak_enemies_bundle};

pub fn create_basic_enemy() -> EnemyBuilder {
    EnemyBuilder::new()
        .with_stats(100, 6, 6, 4)
        .with_sprite(16)
        .with_loot_pool(weak_enemies_bundle())
}

pub fn create_trail_enemy() -> EnemyBuilder {
    EnemyBuilder::new()
        .with_stats(50, 10, 6, 5)
        .with_sprite(20)
        .with_trail(4, 0.1, 10.0)
        .with_loot_pool(medium_enemies_bundle())
}

pub fn create_shooter_enemy() -> EnemyBuilder {
    EnemyBuilder::new()
        .with_stats(100, 8, 0, 10)
        .with_sprite(52)
        .with_shooting(3, 0.5, 2.0, 300.0)
        .with_loot_pool(medium_enemies_bundle())
        .with_ranged_behavior(250.0, 50.0)
}

pub fn create_charging_enemy() -> EnemyBuilder {
    EnemyBuilder::new()
        .with_stats(80, 6, 8, 8)
        .with_sprite(36)
        .with_charge(200, 15, 1.5)
        .with_loot_pool(medium_enemies_bundle())
}

pub fn create_bomber_enemy() -> EnemyBuilder {
    EnemyBuilder::new()
        .with_stats(30, 8, 0, 12)
        .with_sprite(48)
        .with_explosion(100.0, 30)
        .with_loot_pool(strong_enemies_bundle())
}

pub fn create_boss_enemy() -> EnemyBuilder {
    EnemyBuilder::new()
        .with_stats(30000, 5, 10, 50)
        .with_sprite(240)
        .with_trail(8, 0.1, 15.0)
        .with_shooting(5, 0.3, 1.5, 400.0)
        .with_explosion(150.0, 50)
        .with_loot_pool(strong_enemies_bundle())
}
