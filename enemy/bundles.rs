use super::{components::*, types::EnemyType};
use crate::{
    animation::AnimationTimer, configs::SPRITE_SCALE_FACTOR, loot::LootPool,
    resources::GlobalTextureAtlas, utils::InGameEntity,
};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub enemy_type: EnemyType,
    pub sprite_bundle: SpriteBundle,
    pub texture_atlas: TextureAtlas,
    pub animation_timer: AnimationTimer,
    pub in_game_entity: InGameEntity,
    pub lootpool: LootPool,
    pub collider: Collider,
    pub action_timer: PerformAction,
}

impl EnemyBundle {
    pub fn new(
        enemy_type: EnemyType,
        position: Vec3,
        handle: &Res<GlobalTextureAtlas>,
        loot_pool: LootPool,
    ) -> Self {
        let config = enemy_type.get_config();
        Self {
            enemy: Enemy {
                health: config.health,
                speed: config.speed,
                damage: config.damage,
                xp: config.xp,
                enemy_type: enemy_type.clone(),
            },
            enemy_type,
            sprite_bundle: SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            texture_atlas: TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: config.sprite_index,
            },
            animation_timer: AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
            in_game_entity: InGameEntity,
            collider: Collider { radius: 15 },
            lootpool: loot_pool,
            action_timer: PerformAction(Timer::from_seconds(3.0, TimerMode::Repeating), false),
        }
    }
}
