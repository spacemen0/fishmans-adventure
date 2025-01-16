use rand::Rng;

use crate::{
    animation::AnimationTimer,
    armor::{ActiveArmor, Armor, ArmorStats},
    configs::*,
    game_state::GameState,
    gun::{ActiveGun, Gun},
    loot::{Description, Value},
    player::{
        DamageBoost, Defense, Gold, Health, OriginalColor, Player, PlayerInventory, PlayerState,
        Speed,
    },
    potion::{Potion, PotionStats, PotionType},
    resources::{GlobalTextureAtlas, Level, Wave},
    utils::InGameEntity,
};
use bevy::{math::vec3, prelude::*};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Initializing),
            (
                spawn_background,
                spawn_world_decorations,
                spawn_world_edges,
                init_world,
            )
                .chain(),
        );
    }
}

pub fn init_world(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.insert_resource(Wave::default());
    commands.insert_resource(Level::default());

    let player_entity = commands
        .spawn((
            Name::new("Player"),
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout_16x16.clone().unwrap(),
                    index: 0,
                }),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, LAYER2))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Player,
            Health(PLAYER_HEALTH, PLAYER_HEALTH),
            Speed(PLAYER_SPEED),
            Defense(1),
            Gold(100),
            DamageBoost(0),
            OriginalColor(Color::WHITE),
            PlayerState::default(),
            AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
            InGameEntity,
        ))
        .id();

    let default_gun = commands
        .spawn((
            Name::new("DefaultGun"),
            Gun,
            Value(10),
            ActiveGun,
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout_16x16.clone().unwrap(),
                    index: 65,
                }),
                ..default()
            },
            Description {
                name: "Apprentice Gun".to_string(),
                description: format!(
                    "Damage: {}; Speed: {}; Firing Interval: {}; Bullet Per Shot: {}",
                    BULLET_DAMAGE, BULLET_SPEED, FIRING_INTERVAL, NUM_BULLETS_PER_SHOT,
                ),
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, LAYER3))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
        ))
        .id();

    let health_potion = commands
        .spawn((
            Name::new("HealthPotion"),
            Potion,
            Value(5),
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout_16x16.clone().unwrap(),
                    index: 96,
                }),

                ..default()
            },
            Visibility::Hidden,
            Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            PotionStats {
                effect_duration: 0.0,
                effect_amount: 10,
            },
            Description {
                name: "Health Potion".to_string(),
                description: "Amount: 10".to_string(),
            },
            PotionType::Health,
        ))
        .id();
    let speed_potion = commands
        .spawn((
            Name::new("SpeedPotion"),
            Potion,
            Value(5),
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout_16x16.clone().unwrap(),
                    index: 97,
                }),

                ..default()
            },
            Visibility::Hidden,
            Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            PotionStats {
                effect_duration: 5.0,
                effect_amount: 10,
            },
            Description {
                name: "Speed Potion".to_string(),
                description: "Duration: 1.0s; Amount: 10".to_string(),
            },
            PotionType::Speed,
        ))
        .id();

    let basic_armor = commands
        .spawn((
            Name::new("BasicArmor"),
            Armor,
            Value(10),
            ArmorStats {
                defense: 2,
                durability: 15,
            },
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout_16x16.clone().unwrap(),
                    index: 98,
                }),
                ..default()
            },
            ActiveArmor,
            Description {
                name: "Apprentice Armor".to_string(),
                description: "Defense: 2, Durability: 15".to_string(),
            },
            Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Visibility::Hidden,
        ))
        .id();

    let advanced_armor = commands
        .spawn((
            Name::new("AdvancedArmor"),
            Armor,
            Value(10),
            ArmorStats {
                defense: 3,
                durability: 20,
            },
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout_16x16.clone().unwrap(),
                    index: 99,
                }),
                ..default()
            },
            Description {
                name: "Advanced Armor".to_string(),
                description: "Defense: 3, Durability: 20".to_string(),
            },
            Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Visibility::Hidden,
        ))
        .id();

    commands.entity(player_entity).insert(PlayerInventory {
        guns: vec![default_gun],
        active_gun_index: 0,
        health_potions: vec![health_potion],
        speed_potions: vec![speed_potion],
        armors: vec![basic_armor, advanced_armor],
        active_armor_index: 0,
    });

    next_state.set(GameState::Combat);
}

pub fn spawn_background(mut commands: Commands, handle: Res<GlobalTextureAtlas>) {
    let tile_size = TILE_W as f32 * SPRITE_SCALE_FACTOR;
    let width = WW;
    let height = WH;

    for x in (-width as i32..=width as i32).step_by(tile_size as usize) {
        for y in (-height as i32..=height as i32).step_by(tile_size as usize) {
            commands.spawn((
                Name::new("Background"),
                Sprite {
                    image: handle.image.clone().unwrap(),
                    texture_atlas: Some(TextureAtlas {
                        layout: handle.layout_16x16.clone().unwrap(),
                        index: rand::thread_rng().gen_range(8..=13),
                    }),
                    ..default()
                },
                Transform::from_translation(vec3(x as f32, y as f32, LAYER0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                InGameEntity,
            ));
        }
    }
}

fn spawn_world_decorations(mut commands: Commands, handle: Res<GlobalTextureAtlas>) {
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_WORLD_DECORATIONS {
        let x = rng.gen_range((-WW + TILE_W as f32)..(WW - TILE_W as f32));
        let y = rng.gen_range((-WH + TILE_H as f32)..(WH - TILE_H as f32));
        commands.spawn((
            Name::new("Decoration"),
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout_16x16.clone().unwrap(),
                    index: rng.gen_range(111..=116),
                }),
                ..default()
            },
            Transform::from_translation(vec3(x, y, LAYER1))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            InGameEntity,
        ));
    }
}

fn spawn_world_edges(mut commands: Commands, handle: Res<GlobalTextureAtlas>) {
    // Top edge
    for x in (-WW as i32..=WW as i32).step_by((TILE_H as f32 * SPRITE_SCALE_FACTOR) as usize) {
        commands.spawn((
            Name::new("Edge"),
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout_16x16.clone().unwrap(),
                    index: 113,
                }),
                ..default()
            },
            Transform::from_translation(vec3(x as f32, WH, LAYER1))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            InGameEntity,
        ));
    }

    // Bottom edge
    for x in (-WW as i32..=WW as i32).step_by((TILE_H as f32 * SPRITE_SCALE_FACTOR) as usize) {
        commands.spawn((
            Name::new("Edge"),
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout_16x16.clone().unwrap(),
                    index: 113,
                }),
                ..default()
            },
            Transform::from_translation(vec3(x as f32, -WH, LAYER1))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            InGameEntity,
        ));
    }

    // Left edge
    for y in (-WH as i32..=WH as i32).step_by((TILE_H as f32 * SPRITE_SCALE_FACTOR) as usize) {
        commands.spawn((
            Name::new("Edge"),
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout_16x16.clone().unwrap(),
                    index: 113,
                }),
                ..default()
            },
            Transform::from_translation(vec3(-WW, y as f32, LAYER1))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            InGameEntity,
        ));
    }

    // Right edge
    for y in (-WH as i32..=WH as i32).step_by((TILE_H as f32 * SPRITE_SCALE_FACTOR) as usize) {
        commands.spawn((
            Name::new("Edge"),
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout_16x16.clone().unwrap(),
                    index: 113,
                }),
                ..default()
            },
            Transform::from_translation(vec3(WW, y as f32, LAYER1))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            InGameEntity,
        ));
    }
}
