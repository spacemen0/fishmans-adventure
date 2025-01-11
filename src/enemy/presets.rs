use super::builder::EnemyBuilder;
use crate::loot::{medium_enemies_bundle, strong_enemies_bundle, weak_enemies_bundle};

pub fn create_basic_enemy() -> EnemyBuilder {
    EnemyBuilder::new()
        .with_stats(100, 6, 6, 4)
        .with_sprite(16, (16, 16))
        .with_loot_pool(weak_enemies_bundle())
}

pub fn create_trail_enemy() -> EnemyBuilder {
    EnemyBuilder::new()
        .with_stats(50, 10, 6, 5)
        .with_sprite(20, (16, 16))
        .with_trail(4, 0.1, 10.0)
        .with_loot_pool(medium_enemies_bundle())
}

pub fn create_shooter_enemy() -> EnemyBuilder {
    EnemyBuilder::new()
        .with_stats(100, 8, 0, 10)
        .with_sprite(52, (16, 16))
        .with_shooting(3, 2.0, 405.0, 400, 10)
        .with_loot_pool(medium_enemies_bundle())
        .with_ranged_behavior(350.0, 50.0)
}

pub fn create_charging_enemy() -> EnemyBuilder {
    EnemyBuilder::new()
        .with_stats(80, 6, 8, 8)
        .with_sprite(36, (16, 16))
        .with_charge(200, 15, 1.5)
        .with_loot_pool(medium_enemies_bundle())
}

pub fn create_bomber_enemy() -> EnemyBuilder {
    EnemyBuilder::new()
        .with_stats(30, 8, 30, 12)
        .with_sprite(48, (16, 16))
        .with_explosion(100.0, 30)
        .with_loot_pool(strong_enemies_bundle())
}

pub fn create_gurgle_enemy() -> EnemyBuilder {
    EnemyBuilder::new()
        .with_stats(60, 8, 0, 8)
        .with_sprite(32, (16, 16))
        .with_shooting(1, 2.0, 605.0, 300, 30)
        .with_ranged_behavior(550.0, 50.0)
        .with_gurgle_marker()
}

pub fn create_splitting_enemy() -> EnemyBuilder {
    EnemyBuilder::new()
        .with_stats(120, 7, 8, 15)
        .with_sprite(56, (16, 16))
        .with_splitting(3)
        .with_loot_pool(strong_enemies_bundle())
}

pub fn create_midgame_boss_enemy() -> EnemyBuilder {
    EnemyBuilder::new()
        .with_stats(50000, 8, 20, 100)
        .with_sprite(56, (32, 32))
        .with_trail(10, 0.2, 20.0)
        .with_shooting(5, 1.0, 500.0, 300, 15)
        .with_charge(600, 50, 0.6)
        .with_summoning(6, 12, 10.0)
        .with_loot_pool(strong_enemies_bundle())
}
