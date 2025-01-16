use bevy::prelude::*;
use rand::Rng;

use crate::{
    armor::{Armor, ArmorStats},
    configs::{LAYER3, SPRITE_SCALE_FACTOR},
    gun::{BulletStats, Gun, GunStats, GunType},
    potion::{Potion, PotionStats, PotionType},
    utils::{generate_random_cool_name, get_random_position_around, Pickable},
};

#[derive(Component, Default)]
pub struct Value(pub u32);

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
    pub bullet_speed: (u32, u32),
    pub bullet_lifespan: (f32, f32),
    pub bullet_damage: (u32, u32),
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
    pub value: u32,
    pub spawn_fn: fn(
        &mut Commands,
        &Transform,
        Option<Handle<Image>>,
        Option<Handle<TextureAtlasLayout>>,
        LootStatRange,
        u32,
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

pub fn spawn_gun_entity(
    commands: &mut Commands,
    position: Vec3,
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
    stat_range: GunStatRange,
    value: u32,
) -> Entity {
    let mut rng = rand::thread_rng();
    let bullet_stats = BulletStats {
        speed: rng.gen_range(stat_range.bullet_speed.0..=stat_range.bullet_speed.1),
        lifespan: rng.gen_range(stat_range.bullet_lifespan.0..=stat_range.bullet_lifespan.1),
        damage: rng.gen_range(stat_range.bullet_damage.0..=stat_range.bullet_damage.1),
    };
    let (gun_type, gun_stats) = match rng.gen_range(0..3) {
        0 => (
            GunType::FocusedAim,
            GunStats {
                bullets_per_shot: 1,
                firing_interval: rng
                    .gen_range(stat_range.firing_interval.0..=stat_range.firing_interval.1)
                    * 0.5,
                bullet_spread: 0.0,
            },
        ),
        1 => (
            GunType::OmniSpread,
            GunStats {
                bullets_per_shot: (rng
                    .gen_range(stat_range.bullets_per_shot.0..=stat_range.bullets_per_shot.1)
                    as f32
                    * 1.5) as usize,
                firing_interval: rng
                    .gen_range(stat_range.firing_interval.0..=stat_range.firing_interval.1)
                    * 0.8,
                bullet_spread: rng
                    .gen_range(stat_range.bullet_spread.0..=stat_range.bullet_spread.1),
            },
        ),
        2 => (
            GunType::SingleDirectionSpread,
            GunStats {
                bullets_per_shot: rng
                    .gen_range(stat_range.bullets_per_shot.0..=stat_range.bullets_per_shot.1),
                firing_interval: rng
                    .gen_range(stat_range.firing_interval.0..=stat_range.firing_interval.1),
                bullet_spread: rng
                    .gen_range(stat_range.bullet_spread.0..=stat_range.bullet_spread.1),
            },
        ),
        _ => unreachable!(),
    };
    commands
        .spawn((
            Name::new("Gun"),
            Gun,
            Value(value),
            Sprite {
                image,
                texture_atlas: Some(TextureAtlas {
                    layout,
                    index: rng.gen_range(64..67),
                }),
                ..default()
            },
            gun_type,
            Description {
                name: generate_random_cool_name(LootType::Gun),
                description: format!(
                    "Damage: {}; Speed: {}; Firing Interval: {:.2}; Bullet Per Shot: {}",
                    bullet_stats.damage,
                    bullet_stats.speed,
                    gun_stats.firing_interval,
                    gun_stats.bullets_per_shot,
                ),
            },
            Transform::from_translation(position).with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            gun_stats,
            bullet_stats,
        ))
        .id()
}

pub fn spawn_armor_entity(
    commands: &mut Commands,
    position: Vec3,
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
    stat_range: ArmorStatRange,
    value: u32,
) -> Entity {
    let mut rng = rand::thread_rng();
    let armor_stats = ArmorStats {
        defense: rng.gen_range(stat_range.defense.0..=stat_range.defense.1),
        durability: rng.gen_range(stat_range.durability.0..=stat_range.durability.1),
    };
    commands
        .spawn((
            Name::new("Armor"),
            Armor,
            Value(value),
            Sprite {
                image,
                texture_atlas: Some(TextureAtlas {
                    layout,
                    index: rng.gen_range(98..99),
                }),
                ..default()
            },
            Description {
                name: generate_random_cool_name(LootType::Armor),
                description: format!(
                    "Defense: {}; Durability: {}",
                    armor_stats.defense, armor_stats.durability
                ),
            },
            armor_stats,
            Transform::from_translation(position).with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
        ))
        .id()
}

pub fn spawn_potion_entity(
    commands: &mut Commands,
    position: Vec3,
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
    stat_range: PotionStatRange,
    value: u32,
) -> Entity {
    let mut rng = rand::thread_rng();
    let potion_stats = PotionStats {
        effect_duration: rng.gen_range(stat_range.effect_duration.0..=stat_range.effect_duration.1),
        effect_amount: rng.gen_range(stat_range.effect_amount.0..=stat_range.effect_amount.1),
    };
    let potion_type = match rng.gen_range(0..2) {
        0 => (PotionType::Speed, 97),
        1 => (PotionType::Health, 96),
        _ => unreachable!(),
    };
    let name_string = format!("{:?} Potion ", potion_type.clone());
    commands
        .spawn((
            Name::new(name_string),
            Potion,
            Value(value),
            Sprite {
                image,
                texture_atlas: Some(TextureAtlas {
                    layout,
                    index: potion_type.1,
                }),
                ..default()
            },
            Description {
                name: generate_random_cool_name(LootType::Potion),
                description: match potion_type.0 {
                    PotionType::Speed => format!(
                        "Duration: {:.1}s; Amount: {}",
                        potion_stats.effect_duration, potion_stats.effect_amount
                    ),
                    PotionType::Health => format!("Amount: {}", potion_stats.effect_amount),
                },
            },
            potion_stats,
            potion_type.0,
            Transform::from_translation(position).with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
        ))
        .id()
}

fn spawn_gun(
    commands: &mut Commands,
    transform: &Transform,
    image: Option<Handle<Image>>,
    layout: Option<Handle<TextureAtlasLayout>>,
    stat_range: LootStatRange,
    value: u32,
) {
    if let LootStatRange::Gun(range) = stat_range {
        let (x, y) = get_random_position_around(transform.translation.xy(), 30.0..60.0);
        let id = spawn_gun_entity(
            commands,
            Vec3::new(x, y, LAYER3),
            image.unwrap(),
            layout.unwrap(),
            range,
            value,
        );
        commands.entity(id).insert(Pickable);
    }
}

fn spawn_armor(
    commands: &mut Commands,
    transform: &Transform,
    image: Option<Handle<Image>>,
    layout: Option<Handle<TextureAtlasLayout>>,
    stat_range: LootStatRange,
    value: u32,
) {
    if let LootStatRange::Armor(range) = stat_range {
        let (x, y) = get_random_position_around(transform.translation.xy(), 30.0..60.0);
        let id = spawn_armor_entity(
            commands,
            Vec3::new(x, y, LAYER3),
            image.unwrap(),
            layout.unwrap(),
            range,
            value,
        );
        commands.entity(id).insert(Pickable);
    }
}

fn spawn_potion(
    commands: &mut Commands,
    transform: &Transform,
    image: Option<Handle<Image>>,
    layout: Option<Handle<TextureAtlasLayout>>,
    stat_range: LootStatRange,
    value: u32,
) {
    if let LootStatRange::Potion(range) = stat_range {
        let (x, y) = get_random_position_around(transform.translation.xy(), 30.0..60.0);
        let id = spawn_potion_entity(
            commands,
            Vec3::new(x, y, LAYER3),
            image.unwrap(),
            layout.unwrap(),
            range,
            value,
        );
        commands.entity(id).insert(Pickable);
    }
}

pub fn weak_enemies_loots() -> LootPool {
    LootPool {
        items: vec![
            LootDefinition {
                loot_type: LootType::Potion,
                drop_chance: 0.08,
                value: 10,
                spawn_fn: spawn_potion,
                stat_range: LootStatRange::Potion(PotionStatRange {
                    effect_duration: (3.0, 5.0),
                    effect_amount: (5, 10),
                }),
            },
            LootDefinition {
                loot_type: LootType::Gun,
                drop_chance: 0.01,
                value: 20,
                spawn_fn: spawn_gun,
                stat_range: LootStatRange::Gun(GunStatRange {
                    bullets_per_shot: (10, 20),
                    firing_interval: (0.3, 0.5),
                    bullet_spread: (0.15, 0.2),
                    bullet_speed: (10, 20),
                    bullet_lifespan: (1.0, 2.0),
                    bullet_damage: (30, 40),
                }),
            },
        ],
    }
}

pub fn medium_enemies_loots() -> LootPool {
    LootPool {
        items: vec![
            LootDefinition {
                loot_type: LootType::Potion,
                drop_chance: 0.1,
                value: 12,
                spawn_fn: spawn_potion,
                stat_range: LootStatRange::Potion(PotionStatRange {
                    effect_duration: (4.0, 7.0),
                    effect_amount: (7, 12),
                }),
            },
            LootDefinition {
                loot_type: LootType::Armor,
                drop_chance: 0.03,
                value: 25,
                spawn_fn: spawn_armor,
                stat_range: LootStatRange::Armor(ArmorStatRange {
                    defense: (1, 3),
                    durability: (15, 30),
                }),
            },
            LootDefinition {
                loot_type: LootType::Gun,
                drop_chance: 0.04,
                value: 30,
                spawn_fn: spawn_gun,
                stat_range: LootStatRange::Gun(GunStatRange {
                    bullets_per_shot: (15, 30),
                    firing_interval: (0.2, 0.4),
                    bullet_spread: (0.1, 0.15),
                    bullet_damage: (50, 100),
                    bullet_lifespan: (1.5, 3.0),
                    bullet_speed: (20, 30),
                }),
            },
        ],
    }
}

pub fn strong_enemies_loots() -> LootPool {
    LootPool {
        items: vec![
            LootDefinition {
                loot_type: LootType::Potion,
                drop_chance: 0.3,
                value: 15,
                spawn_fn: spawn_potion,
                stat_range: LootStatRange::Potion(PotionStatRange {
                    effect_duration: (6.0, 10.0),
                    effect_amount: (8, 15),
                }),
            },
            LootDefinition {
                loot_type: LootType::Gun,
                drop_chance: 0.1,
                value: 40,
                spawn_fn: spawn_gun,
                stat_range: LootStatRange::Gun(GunStatRange {
                    bullets_per_shot: (40, 60),
                    firing_interval: (0.1, 0.3),
                    bullet_spread: (0.05, 0.15),
                    bullet_damage: (70, 120),
                    bullet_lifespan: (2.0, 4.0),
                    bullet_speed: (30, 50),
                }),
            },
            LootDefinition {
                loot_type: LootType::Armor,
                drop_chance: 0.1,
                value: 40,
                spawn_fn: spawn_armor,
                stat_range: LootStatRange::Armor(ArmorStatRange {
                    defense: (2, 5),
                    durability: (20, 40),
                }),
            },
        ],
    }
}

pub fn boss_enemy_loots() -> LootPool {
    LootPool {
        items: vec![
            LootDefinition {
                loot_type: LootType::Potion,
                drop_chance: 0.5,
                value: 20,
                spawn_fn: spawn_potion,
                stat_range: LootStatRange::Potion(PotionStatRange {
                    effect_duration: (10.0, 20.0),
                    effect_amount: (10, 20),
                }),
            },
            LootDefinition {
                loot_type: LootType::Gun,
                drop_chance: 0.2,
                value: 50,
                spawn_fn: spawn_gun,
                stat_range: LootStatRange::Gun(GunStatRange {
                    bullets_per_shot: (60, 80),
                    firing_interval: (0.1, 0.2),
                    bullet_spread: (0.05, 0.1),
                    bullet_damage: (140, 200),
                    bullet_lifespan: (3.0, 5.0),
                    bullet_speed: (50, 80),
                }),
            },
            LootDefinition {
                loot_type: LootType::Armor,
                drop_chance: 0.2,
                value: 50,
                spawn_fn: spawn_armor,
                stat_range: LootStatRange::Armor(ArmorStatRange {
                    defense: (3, 7),
                    durability: (30, 60),
                }),
            },
        ],
    }
}
