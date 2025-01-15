use bevy::prelude::*;
use rand::{seq::SliceRandom, Rng};

use crate::{
    collision::EnemyKdTree,
    configs::{LAYER1, MAX_DEFENSE, SPRITE_SCALE_FACTOR, TILE_W, WH, WW},
    loot::LootType,
};

#[derive(Component, Default)]
pub struct InGameEntity;

#[derive(Component)]
pub struct Pickable;

pub fn calculate_enemies_per_wave(_wave_number: u32) -> u32 {
    let base_enemies = 1;
    //let increase = (wave_number as f32 * 0.5).floor() as u32 * 3;
    let increase = 2;
    base_enemies + increase
}

pub fn calculate_health_increase(level: u32) -> u32 {
    let base_health_increase = 10.0;
    let exponential_factor: f32 = 1.05;
    let diminishing_return_factor: f32 = 0.95;

    let health_increase = base_health_increase
        * (exponential_factor.powi(level as i32))
        * diminishing_return_factor.powi(level as i32);

    health_increase.round() as u32
}

pub fn calculate_defense_increase(level: u32) -> u32 {
    let base_defense_increase = 2.0;
    let exponential_factor: f32 = 1.03;
    let diminishing_return_factor: f32 = 0.97;

    let defense_increase = base_defense_increase
        * (exponential_factor.powi(level as i32))
        * diminishing_return_factor.powi(level as i32);

    defense_increase.round() as u32
}

pub fn calculate_damage_boost_increase(level: u32) -> u32 {
    let base_damage_boost: f32 = 2.0;
    let target_damage_boost = 10.0;
    let target_level = 20.0;
    let exponential_factor =
        (target_damage_boost / base_damage_boost).powf(1.0 / (target_level - 1.0));
    let diminishing_return_factor: f32 = 0.97;

    let damage_boost_increase = base_damage_boost
        * (exponential_factor.powi(level as i32))
        * diminishing_return_factor.powi(level as i32);

    damage_boost_increase.round() as u32
}

pub fn get_random_position_around(pos: Vec2, dist_range: std::ops::Range<f32>) -> (f32, f32) {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let dist = rng.gen_range(dist_range);

    let offset_x = angle.cos() * dist;
    let offset_y = angle.sin() * dist;

    let random_x = pos.x + offset_x;
    let random_y = pos.y + offset_y;

    (random_x.clamp(-WW, WW), random_y.clamp(-WH, WH))
}

pub fn clamp_position(position: &mut Vec3) {
    let edge_offset = TILE_W as f32 * SPRITE_SCALE_FACTOR;
    position.x = position.x.clamp(-WW + edge_offset, WW - edge_offset);
    position.y = position.y.clamp(-WH + edge_offset, WH - edge_offset);
    position.z = LAYER1;
}

pub fn cleanup_entities(mut commands: Commands, all_entities: Query<Entity, With<InGameEntity>>) {
    for entity in all_entities.iter() {
        if let Some(entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn_recursive();
        }
    }
}

pub fn apply_movement(position: &mut Vec3, mut movement: Vec2, layer: f32) {
    const MARGIN: f32 = 50.0;
    const REPEL_MARGIN: f32 = 120.0;
    const REPEL_STRENGTH: f32 = 2.0;

    let mut repulsion = Vec2::ZERO;

    if position.x < -WW + REPEL_MARGIN {
        let force = (-WW + REPEL_MARGIN - position.x) / (REPEL_MARGIN - MARGIN);
        repulsion.x += force * REPEL_STRENGTH;
    }
    if position.x > WW - REPEL_MARGIN {
        let force = (WW - REPEL_MARGIN - position.x) / (REPEL_MARGIN - MARGIN);
        repulsion.x += force * REPEL_STRENGTH;
    }
    if position.y < -WH + REPEL_MARGIN {
        let force = (-WH + REPEL_MARGIN - position.y) / (REPEL_MARGIN - MARGIN);
        repulsion.y += force * REPEL_STRENGTH;
    }
    if position.y > WH - REPEL_MARGIN {
        let force = (WH - REPEL_MARGIN - position.y) / (REPEL_MARGIN - MARGIN);
        repulsion.y += force * REPEL_STRENGTH;
    }

    movement += repulsion;

    position.x = (position.x + movement.x).clamp(-WW + MARGIN, WW - MARGIN);
    position.y = (position.y + movement.y).clamp(-WH + MARGIN, WH - MARGIN);
    position.z = layer;
}

pub fn get_nearest_enemy_position(player_pos: Vec2, enemy_kd_tree: &EnemyKdTree) -> Option<Vec2> {
    enemy_kd_tree
        .0
        .nearest(&[player_pos.x, player_pos.y])
        .into_iter()
        .next()
        .map(|nearest_enemy| Vec2::new(nearest_enemy.item.pos[0], nearest_enemy.item.pos[1]))
}

pub fn calculate_defense_percentage(defense: u32) -> f32 {
    let defense = defense.min(MAX_DEFENSE);
    let max_percentage = 0.8;
    let scaled_defense = (defense as f32).sqrt() / (MAX_DEFENSE as f32).sqrt();
    scaled_defense * max_percentage
}

pub fn generate_random_cool_name(loot_type: LootType) -> String {
    let adjectives = [
        "Epic",
        "Legendary",
        "Mystic",
        "Ancient",
        "Enchanted",
        "Fierce",
        "Glorious",
        "Heroic",
        "Mighty",
        "Noble",
        "Powerful",
        "Radiant",
        "Savage",
        "Valiant",
        "Vicious",
    ];

    let gun_nouns = [
        "Blaster", "Cannon", "Rifle", "Pistol", "Shotgun", "Sniper", "Launcher", "Repeater",
    ];

    let armor_nouns = [
        "Armor",
        "Shield",
        "Helmet",
        "Chestplate",
        "Gauntlets",
        "Greaves",
        "Boots",
        "Bracers",
    ];

    let potion_nouns = [
        "Potion",
        "Elixir",
        "Brew",
        "Tonic",
        "Draught",
        "Serum",
        "Concoction",
        "Mixture",
    ];

    let mut rng = rand::thread_rng();
    let adjective = adjectives.choose(&mut rng).unwrap();
    let noun = match loot_type {
        LootType::Gun => gun_nouns.choose(&mut rng).unwrap(),
        LootType::Armor => armor_nouns.choose(&mut rng).unwrap(),
        LootType::Potion => potion_nouns.choose(&mut rng).unwrap(),
    };

    format!("{} {}", adjective, noun)
}
