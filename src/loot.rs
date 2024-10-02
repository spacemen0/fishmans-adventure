use bevy::prelude::*;
use rand::Rng;

use crate::{
    armor::{Armor, ArmorBundle, ArmorStats},
    gun::GunBundle,
    potion::{Potion, PotionBundle, PotionStats, PotionType},
    world::InGameEntity,
    GlobalTextureAtlas, SPRITE_SCALE_FACTOR,
};

#[derive(Clone)]
pub enum LootType {
    Gun,
    Armor,
    Potion,
}

#[derive(Clone)]
pub struct LootDefinition {
    pub loot_type: LootType,
    pub drop_chance: f32,
    pub spawn_fn: fn(&mut Commands, &Transform, handle: Res<GlobalTextureAtlas>),
}

#[derive(Component)]
pub struct LootPool {
    pub items: Vec<LootDefinition>,
}

impl LootPool {
    pub fn get_random_loot(&self) -> Option<LootDefinition> {
        let mut rng = rand::thread_rng();
        for item in &self.items {
            let roll: f32 = rng.gen();
            if roll < item.drop_chance {
                return Some(item.clone());
            }
        }
        None
    }
}

fn spawn_gun(commands: &mut Commands, transform: &Transform, handle: Res<GlobalTextureAtlas>) {
    commands.spawn((
        GunBundle {
            sprite_bundle: SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: *transform,
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            layout: handle.layout.clone().unwrap(),
            index: 17,
        },
    ));
}

fn spawn_armor(commands: &mut Commands, transform: &Transform, handle: Res<GlobalTextureAtlas>) {
    commands.spawn((
        ArmorBundle {
            armor: Armor,
            armor_stats: ArmorStats {
                defense: 2,
                durability: 20,
            },
            in_game_entity: InGameEntity,
        },
        SpriteBundle {
            texture: handle.image.clone().unwrap(),
            transform: *transform,
            ..default()
        },
        TextureAtlas {
            layout: handle.layout.clone().unwrap(),
            index: 58,
        },
    ));
}

fn spawn_potion(commands: &mut Commands, transform: &Transform, handle: Res<GlobalTextureAtlas>) {
    commands.spawn((
        PotionBundle {
            sprite_bundle: SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                visibility: Visibility::Hidden,
                ..default()
            },
            potion: Potion,
            potion_stats: PotionStats {
                effect_duration: 5.0,
                effect_amount: 10,
            },
            potion_type: PotionType::Speed,
            in_game_entity: InGameEntity,
        },
        SpriteBundle {
            texture: handle.image.clone().unwrap(),
            transform: *transform,
            ..default()
        },
        TextureAtlas {
            layout: handle.layout.clone().unwrap(),
            index: 57,
        },
    ));
}

pub fn weak_enemies_bundle() -> LootPool {
    LootPool {
        items: vec![
            LootDefinition {
                loot_type: LootType::Potion,
                drop_chance: 0.2,
                spawn_fn: spawn_potion,
            },
            LootDefinition {
                loot_type: LootType::Gun,
                drop_chance: 0.05,
                spawn_fn: spawn_gun,
            },
        ],
    }
}

pub fn medium_enemies_bundle() -> LootPool {
    LootPool {
        items: vec![
            LootDefinition {
                loot_type: LootType::Potion,
                drop_chance: 0.3,
                spawn_fn: spawn_potion,
            },
            LootDefinition {
                loot_type: LootType::Armor,
                drop_chance: 0.1,
                spawn_fn: spawn_armor,
            },
        ],
    }
}

pub fn strong_enemies_bundle() -> LootPool {
    LootPool {
        items: vec![
            LootDefinition {
                loot_type: LootType::Potion,
                drop_chance: 0.8,
                spawn_fn: spawn_potion,
            },
            LootDefinition {
                loot_type: LootType::Gun,
                drop_chance: 0.3,
                spawn_fn: spawn_gun,
            },
            LootDefinition {
                loot_type: LootType::Armor,
                drop_chance: 0.4,
                spawn_fn: spawn_armor,
            },
        ],
    }
}
