use rand::Rng;

use crate::{
    animation::AnimationTimer,
    armor::{Armor, ArmorBundle, ArmorStats},
    configs::*,
    gun::{BulletStats, GunBundle, GunStats, GunType},
    player::{Defense, Health, Player, PlayerInventory, PlayerState, Speed},
    potion::{Potion, PotionBundle, PotionStats, PotionType},
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
        )
        .add_systems(OnExit(GameState::Combat), despawn_all_game_entities);
    }
}

fn init_world(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.insert_resource(Wave::default());
    commands.insert_resource(Level::default());
    // Spawn player
    let player_entity = commands
        .spawn((
            SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 0,
            },
            Player,
            Health(PLAYER_HEALTH),
            Speed(PLAYER_SPEED),
            Defense(1),
            PlayerState::default(),
            AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
            InGameEntity,
        ))
        .id();

    // Spawn first gun
    let gun1 = commands
        .spawn((GunBundle {
            sprite_bundle: SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            texture_bundle: TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 65,
            },
            ..default()
        },))
        .id();

    // Spawn second gun
    let gun2 = commands
        .spawn((GunBundle {
            sprite_bundle: SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                visibility: Visibility::Hidden,
                ..default()
            },
            gun_type: GunType::Gun1,
            gun_stats: GunStats {
                bullets_per_shot: 20,
                firing_interval: 0.1,
                bullet_spread: 0.3,
            },
            bullet_stats: BulletStats {
                speed: 30,
                damage: 100,
                lifespan: 0.5,
            },
            texture_bundle: TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 66,
            },
            ..default()
        },))
        .id();
    let potion1 = commands
        .spawn((PotionBundle {
            sprite_bundle: SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                visibility: Visibility::Hidden,
                ..default()
            },
            potion: Potion,
            potion_stats: PotionStats {
                effect_duration: 2.0,
                effect_amount: 10,
            },
            potion_type: PotionType::Health,
            in_game_entity: InGameEntity,
            texture_bundle: TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 98,
            },
        },))
        .id();
    let potion2 = commands
        .spawn(PotionBundle {
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
            texture_bundle: TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 98,
            },
        })
        .id();

    // Add both guns to the player's inventory
    let armor1 = commands
        .spawn((ArmorBundle {
            armor: Armor,
            armor_stats: ArmorStats {
                defense: 2,
                durability: 20,
            },
            sprite_bundle: SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                visibility: Visibility::Hidden,
                ..default()
            },
            in_game_entity: InGameEntity,
            texture_bundle: TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 99,
            },
        },))
        .id();

    let armor2 = commands
        .spawn(ArmorBundle {
            armor: Armor,
            armor_stats: ArmorStats {
                defense: 3,
                durability: 30,
            },
            sprite_bundle: SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                visibility: Visibility::Hidden,
                ..default()
            },
            in_game_entity: InGameEntity,
            texture_bundle: TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 99,
            },
        })
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
            SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_translation(vec3(x, y, LAYER0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: rng.gen_range(111..=116),
            },
            InGameEntity,
        ));
    }
}

fn spawn_world_edges(mut commands: Commands, handle: Res<GlobalTextureAtlas>) {
    // Top edge
    for x in (-WW as i32..=WW as i32).step_by((TILE_H as f32 * SPRITE_SCALE_FACTOR) as usize) {
        commands.spawn((
            SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_translation(vec3(x as f32, WH, LAYER0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 113,
            },
            InGameEntity,
        ));
    }

    // Bottom edge
    for x in
        (-WW as i32..=WW as i32).step_by((TILE_H as f32 * SPRITE_SCALE_FACTOR) as usize)
    {
        commands.spawn((
            SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_translation(vec3(x as f32, -WH, LAYER0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 113,
            },
            InGameEntity,
        ));
    }

    // Left edge
    for y in (-WH as i32..=WH as i32).step_by((TILE_H as f32 * SPRITE_SCALE_FACTOR) as usize) {
        commands.spawn((
            SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_translation(vec3(-WW, y as f32, LAYER0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 113,
            },
            InGameEntity,
        ));
    }

    // Right edge
    for y in (-WH as i32..=WH as i32).step_by((TILE_H as f32 * SPRITE_SCALE_FACTOR) as usize) {
        commands.spawn((
            SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_translation(vec3(WW, y as f32, LAYER0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 113,
            },
            InGameEntity,
        ));
    }
}

fn despawn_all_game_entities(
    mut commands: Commands,
    all_entities: Query<Entity, With<InGameEntity>>,
    player_query: Query<Entity, With<Player>>,
    next_state: Res<State<GameState>>,
) {
    if *next_state.get() != GameState::Paused {
        for e in all_entities.iter() {
            if let Some(entity) = commands.get_entity(e) {
                entity.despawn_recursive();
            }
        }
    }

    // Ensure the player entity is not despawned
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).remove::<InGameEntity>();
    }
}
