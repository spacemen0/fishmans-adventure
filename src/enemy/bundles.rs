use bevy::prelude::*;

use super::components::*;
use crate::{
    animation::AnimationTimer, configs::SPRITE_SCALE_FACTOR, loot::LootPool,
    resources::GlobalTextureAtlas, utils::InGameEntity,
};

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub sprite_bundle: SpriteBundle,
    pub texture_atlas: TextureAtlas,
    pub animation_timer: AnimationTimer,
    pub in_game_entity: InGameEntity,
    pub lootpool: LootPool,
    pub collider: Collider,
}

impl EnemyBundle {
    pub fn new(
        health: u32,
        speed: u32,
        damage: u32,
        xp: u32,
        sprite_index: usize,
        position: Vec3,
        handle: &GlobalTextureAtlas,
        loot_pool: LootPool,
    ) -> Self {
        Self {
            enemy: Enemy {
                health,
                speed,
                damage,
                xp,
            },
            sprite_bundle: SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            texture_atlas: TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: sprite_index,
            },
            animation_timer: AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
            in_game_entity: InGameEntity,
            collider: Collider { radius: 15 },
            lootpool: loot_pool,
        }
    }
}
