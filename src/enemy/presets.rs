use super::builder::EnemyBuilder;
use crate::loot::{
    boss_enemy_bundle, medium_enemies_bundle, strong_enemies_bundle, weak_enemies_bundle,
};

pub fn create_basic_enemy() -> EnemyBuilder {
    EnemyBuilder::new()
        .with_stats(100, 6, 8, 10)
        .with_sprite(16, (16, 16))
        .with_loot_pool(weak_enemies_bundle())
}

pub fn create_charging_enemy() -> EnemyBuilder {
    EnemyBuilder::new()
        .with_stats(150, 6, 10, 20)
        .with_sprite(36, (16, 16))
        .with_charge(400, 25, 1.5, 5.0)
        .with_loot_pool(weak_enemies_bundle())
}

pub fn create_trail_enemy() -> EnemyBuilder {
    EnemyBuilder::new()
        .with_stats(50, 8, 3, 15)
        .with_sprite(20, (16, 16))
        .with_trail(6, 0.05, 10.0, 3.0)
        .with_loot_pool(weak_enemies_bundle())
}

pub fn create_splitting_enemy() -> EnemyBuilder {
    EnemyBuilder::new()
        .with_stats(120, 3, 4, 5)
        .with_sprite(56, (16, 16))
        .with_splitting(3)
        .with_loot_pool(weak_enemies_bundle())
}

pub fn create_shooter_enemy() -> EnemyBuilder {
    EnemyBuilder::new()
        .with_stats(100, 4, 0, 25)
        .with_sprite(52, (16, 16))
        .with_shooting(3, 2.0, 705.0, 700, 10)
        .with_loot_pool(medium_enemies_bundle())
        .with_ranged_behavior(650.0, 50.0)
}

pub fn create_bomber_enemy() -> EnemyBuilder {
    EnemyBuilder::new()
        .with_stats(30, 9, 30, 25)
        .with_sprite(48, (16, 16))
        .with_explosion(140.0, 14)
        .with_loot_pool(strong_enemies_bundle())
}

pub fn create_gurgle_enemy() -> EnemyBuilder {
    EnemyBuilder::new()
        .with_stats(60, 4, 0, 35)
        .with_sprite(32, (16, 16))
        .with_shooting(1, 1.5, 1205.0, 600, 14)
        .with_ranged_behavior(1150.0, 50.0)
        .with_gurgle_marker()
        .with_loot_pool(strong_enemies_bundle())
}

pub fn create_midgame_boss_enemy() -> EnemyBuilder {
    EnemyBuilder::new()
        .with_stats(35000, 9, 20, 200)
        .with_sprite(56, (32, 32))
        .with_shooting(5, 1.0, 800.0, 300, 15)
        .with_charge(600, 50, 0.6, 3.0)
        .with_summoning(8, 14, 10.0)
        .with_loot_pool(boss_enemy_bundle())
}
