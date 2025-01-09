use iyes_perf_ui::prelude::PerfUiDefaultEntries;
use rand::Rng;

use crate::{
    animation::AnimationTimer,
    armor::{Armor, ArmorStats},
    configs::*,
    gun::{ActiveGun, BulletStats, Gun, GunStats, GunType},
    player::{Defense, Health, OriginalColor, Player, PlayerInventory, PlayerState, Speed},
    potion::{Potion, PotionStats, PotionType},
    resources::{GlobalTextureAtlas, Level, Wave},
    state::GameState,
    utils::InGameEntity,
};
use bevy::{math::vec3, prelude::*};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Initializing),
            (init_world, spawn_world_decorations, spawn_world_edges),
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
    #[cfg(not(target_arch = "wasm32"))]
    commands.spawn((
        PerfUiDefaultEntries::default(),
        Name::new("Debug Ui"),
        InGameEntity,
    ));
    // Spawn player
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
            Transform::from_translation(Vec3::new(0.0, 0.0, LAYER1))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Player,
            Health(PLAYER_HEALTH, 200),
            Speed(PLAYER_SPEED),
            Defense(1),
            OriginalColor(Color::WHITE),
            PlayerState::default(),
            AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
            InGameEntity,
        ))
        .id();
    // Spawn first gun
    let gun1 = commands
        .spawn((
            Name::new("Gun1"),
            Gun,
            ActiveGun,
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout_16x16.clone().unwrap(),
                    index: 65,
                }),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, LAYER2))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
        ))
        .id();

    // Spawn second gun
    let gun2 = commands
        .spawn((
            Name::new("Gun2"),
            Gun,
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout_16x16.clone().unwrap(),
                    index: 66,
                }),
                ..default()
            },
            Visibility::Hidden,
            Transform::from_translation(Vec3::new(0.0, 0.0, LAYER2))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            GunType::Gun1,
            GunStats {
                bullets_per_shot: 20,
                firing_interval: 0.1,
                bullet_spread: 0.3,
            },
            BulletStats {
                speed: 30,
                damage: 100,
                lifespan: 0.5,
            },
        ))
        .id();
    let potion1 = commands
        .spawn((
            Name::new("Potion1"),
            Potion,
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout_16x16.clone().unwrap(),
                    index: 98,
                }),

                ..default()
            },
            Visibility::Hidden,
            Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            PotionStats {
                effect_duration: 2.0,
                effect_amount: 10,
            },
            PotionType::Health,
        ))
        .id();
    let potion2 = commands
        .spawn((
            Potion,
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout_16x16.clone().unwrap(),
                    index: 98,
                }),

                ..default()
            },
            Visibility::Hidden,
            Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            PotionStats {
                effect_duration: 5.0,
                effect_amount: 10,
            },
            PotionType::Health,
        ))
        .id();

    // Add both guns to the player's inventory
    let armor1 = commands
        .spawn((
            Name::new("Armor1"),
            Armor,
            ArmorStats {
                defense: 2,
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
            Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Visibility::Hidden,
        ))
        .id();

    let armor2 = commands
        .spawn((
            Name::new("Armor1"),
            Armor,
            ArmorStats {
                defense: 3,
                durability: 30,
            },
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout_16x16.clone().unwrap(),
                    index: 99,
                }),
                ..default()
            },
            Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Visibility::Hidden,
        ))
        .id();

    // Add guns, potions, and armors to the player's inventory
    commands.entity(player_entity).insert(PlayerInventory {
        guns: vec![gun1, gun2],
        active_gun_index: 0,
        health_potions: vec![potion1],
        speed_potions: vec![potion2],
        armors: vec![armor1, armor2],
        active_armor_index: 0,
    });

    next_state.set(GameState::Combat);
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
            Transform::from_translation(vec3(x, y, LAYER0))
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
            Transform::from_translation(vec3(x as f32, WH, LAYER0))
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
            Transform::from_translation(vec3(x as f32, -WH, LAYER0))
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
            Transform::from_translation(vec3(-WW, y as f32, LAYER0))
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
            Transform::from_translation(vec3(WW, y as f32, LAYER0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            InGameEntity,
        ));
    }
}
