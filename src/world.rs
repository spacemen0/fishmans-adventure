use bevy::math::vec3;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use gun::GunBundle;
use player::InvulnerableTimer;
use rand::Rng;

use crate::animation::AnimationTimer;
use crate::player::{Health, Player, PlayerState};
use crate::*;
use crate::{state::GameState, GlobalTextureAtlas};

pub struct WorldPlugin;

#[derive(Component)]
pub struct InGameEntity; //entities that spawn with this will be cleared after each game run

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::GameInit),
            (init_world, spawn_world_decorations),
        )
        .add_systems(OnExit(GameState::InGame), despawn_all_game_entities);
    }
}

fn init_world(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.insert_resource(Wave::default());
    commands.spawn((
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
        InvulnerableTimer(Stopwatch::new()),
        PlayerState::default(),
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
        InGameEntity,
    ));
    commands.spawn((
        GunBundle {
            sprite_bundle: SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            layout: handle.layout.clone().unwrap(),
            index: 17,
        },
    ));

    next_state.set(GameState::InGame);
}

fn spawn_world_decorations(mut commands: Commands, handle: Res<GlobalTextureAtlas>) {
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_WORLD_DECORATIONS {
        let x = rng.gen_range(-WORLD_W..WORLD_W);
        let y = rng.gen_range(-WORLD_H..WORLD_H);
        commands.spawn((
            SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_translation(vec3(x, y, 0.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: rng.gen_range(24..=25),
            },
            InGameEntity,
        ));
    }
}

fn despawn_all_game_entities(
    mut commands: Commands,
    all_entities: Query<Entity, With<InGameEntity>>,
    next_state: Res<State<GameState>>,
) {
    if *next_state.get() != GameState::Paused {
        for e in all_entities.iter() {
            if let Some(entity) = commands.get_entity(e) {
                entity.despawn_recursive();
            }
        }
    }
}
