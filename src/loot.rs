use bevy::prelude::*;
use rand::Rng;

use crate::{
    armor::{Armor, ArmorStats},
    configs::{LAYER2, SPRITE_SCALE_FACTOR},
    gun::{Gun, GunStats},
    potion::{Potion, PotionStats, PotionType},
    utils::{get_random_position_around, Pickable},
};

#[derive(Clone)]
pub enum LootType {
    Gun,
    Armor,
    Potion,
}
#[derive(Clone)]
pub struct GunStatRange {
    pub bullets_per_shot: (usize, usize),
    pub firing_interval: (f32, f32),
    pub bullet_spread: (f32, f32),
}

#[derive(Clone)]
pub struct ArmorStatRange {
    pub defense: (u32, u32),
    pub durability: (u32, u32),
}

#[derive(Clone)]
pub struct PotionStatRange {
    pub effect_duration: (f32, f32),
    pub effect_amount: (u32, u32),
}

#[derive(Clone)]
pub enum LootStatRange {
    Gun(GunStatRange),
    Armor(ArmorStatRange),
    Potion(PotionStatRange),
    None,
}

#[derive(Clone)]
pub struct LootDefinition {
    pub loot_type: LootType,
    pub drop_chance: f32,
    pub spawn_fn: fn(
        &mut Commands,
        &Transform,
        Option<Handle<Image>>,
        Option<Handle<TextureAtlasLayout>>,
        LootStatRange,
    ),
    pub stat_range: LootStatRange,
}

#[derive(Component)]
pub struct MovingToPlayer;

#[derive(Component)]
pub struct ReadyForPickup;

#[derive(Component)]
pub struct Description {
    pub name: String,
    pub description: String,
}

impl Default for Description {
    fn default() -> Self {
        Description {
            name: String::from("Unnamed Item"),
            description: String::from("No description available."),
        }
    }
}

#[derive(Component, Default)]
pub struct LootPool {
    pub items: Vec<LootDefinition>,
}

impl LootPool {
    pub fn get_random_loots(&self) -> Vec<LootDefinition> {
        let mut rng = rand::thread_rng();
        let mut successful_loots = Vec::new();

        for item in &self.items {
            let roll: f32 = rng.gen();
            if roll < item.drop_chance {
                successful_loots.push(item.clone());
            }
        }

        successful_loots
    }
}

fn spawn_gun(
    commands: &mut Commands,
    transform: &Transform,
    image: Option<Handle<Image>>,
    layout: Option<Handle<TextureAtlasLayout>>,
    stat_range: LootStatRange,
) {
    if let LootStatRange::Gun(range) = stat_range {
        let mut rng = rand::thread_rng();
        let gun_stats = GunStats {
            bullets_per_shot: rng.gen_range(range.bullets_per_shot.0..=range.bullets_per_shot.1),
            firing_interval: rng.gen_range(range.firing_interval.0..=range.firing_interval.1),
            bullet_spread: rng.gen_range(range.bullet_spread.0..=range.bullet_spread.1),
        };
        let (x, y) = get_random_position_around(transform.translation.xy(), 30.0..60.0);
        commands.spawn((
            Name::new("Gun"),
            Gun,
            Sprite {
                image: image.unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: layout.unwrap(),
                    index: 97,
                }),

                ..default()
            },
            Transform::from_translation(Vec3::new(x, y, LAYER2))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            gun_stats,
            Pickable,
        ));
    }
}

fn spawn_armor(
    commands: &mut Commands,
    transform: &Transform,
    image: Option<Handle<Image>>,
    layout: Option<Handle<TextureAtlasLayout>>,
    stat_range: LootStatRange,
) {
    if let LootStatRange::Armor(range) = stat_range {
        let mut rng = rand::thread_rng();
        let armor_stats = ArmorStats {
            defense: rng.gen_range(range.defense.0..=range.defense.1),
            durability: rng.gen_range(range.durability.0..=range.durability.1),
        };
        let (x, y) = get_random_position_around(transform.translation.xy(), 30.0..60.0);
        commands.spawn((
            Name::new("Armor"),
            Armor,
            armor_stats,
            Sprite {
                image: image.unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: layout.unwrap(),
                    index: 95,
                }),
                ..default()
            },
            Transform::from_translation(Vec3::new(x, y, LAYER2))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Pickable,
        ));
    }
}

fn spawn_potion(
    commands: &mut Commands,
    transform: &Transform,
    image: Option<Handle<Image>>,
    layout: Option<Handle<TextureAtlasLayout>>,
    stat_range: LootStatRange,
) {
    if let LootStatRange::Potion(range) = stat_range {
        let mut rng = rand::thread_rng();
        let potion_stats = PotionStats {
            effect_duration: rng.gen_range(range.effect_duration.0..=range.effect_duration.1),
            effect_amount: rng.gen_range(range.effect_amount.0..=range.effect_amount.1),
        };
        let potion_type = match rng.gen_range(0..2) {
            0 => PotionType::Speed,
            1 => PotionType::Health,
            _ => unreachable!(),
        };
        let name_string = format!("{:?} Potion ", potion_type.clone());
        let (x, y) = get_random_position_around(transform.translation.xy(), 30.0..60.0);
        commands.spawn((
            Name::new(name_string),
            Potion,
            Sprite {
                image: image.unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: layout.unwrap(),
                    index: 96,
                }),
                ..default()
            },
            potion_stats,
            potion_type,
            Transform::from_translation(Vec3::new(x, y, LAYER2))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Pickable,
        ));
    }
}

pub fn weak_enemies_bundle() -> LootPool {
    LootPool {
        items: vec![
            LootDefinition {
                loot_type: LootType::Potion,
                drop_chance: 0.2,
                spawn_fn: spawn_potion,
                stat_range: LootStatRange::Potion(PotionStatRange {
                    effect_duration: (3.0, 5.0),
                    effect_amount: (5, 10),
                }),
            },
            LootDefinition {
                loot_type: LootType::Gun,
                drop_chance: 0.05,
                spawn_fn: spawn_gun,
                stat_range: LootStatRange::Gun(GunStatRange {
                    bullets_per_shot: (3, 5),
                    firing_interval: (0.3, 0.5),
                    bullet_spread: (0.15, 0.2),
                }),
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
                stat_range: LootStatRange::Potion(PotionStatRange {
                    effect_duration: (4.0, 7.0),
                    effect_amount: (7, 12),
                }),
            },
            LootDefinition {
                loot_type: LootType::Armor,
                drop_chance: 0.1,
                spawn_fn: spawn_armor,
                stat_range: LootStatRange::Armor(ArmorStatRange {
                    defense: (1, 3),
                    durability: (15, 30),
                }),
            },
            LootDefinition {
                loot_type: LootType::Gun,
                drop_chance: 0.1,
                spawn_fn: spawn_gun,
                stat_range: LootStatRange::Gun(GunStatRange {
                    bullets_per_shot: (5, 8),
                    firing_interval: (0.2, 0.4),
                    bullet_spread: (0.1, 0.15),
                }),
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
                stat_range: LootStatRange::Potion(PotionStatRange {
                    effect_duration: (5.0, 10.0),
                    effect_amount: (8, 15),
                }),
            },
            LootDefinition {
                loot_type: LootType::Gun,
                drop_chance: 0.99,
                spawn_fn: spawn_gun,
                stat_range: LootStatRange::Gun(GunStatRange {
                    bullets_per_shot: (8, 12),
                    firing_interval: (0.1, 0.3),
                    bullet_spread: (0.05, 0.15),
                }),
            },
            LootDefinition {
                loot_type: LootType::Armor,
                drop_chance: 0.4,
                spawn_fn: spawn_armor,
                stat_range: LootStatRange::Armor(ArmorStatRange {
                    defense: (2, 5),
                    durability: (20, 40),
                }),
            },
        ],
    }
}
