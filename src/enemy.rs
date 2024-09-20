use bevy::utils::Duration;
use std::f32::consts::PI;

use bevy::math::vec3;
use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::Rng;

use crate::animation::AnimationTimer;
use crate::player::Player;
use crate::resources::Wave;
use crate::state::GameState;
use crate::world::GameEntity;
use crate::*;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
    pub damage: f32,
    pub traits: Vec<EnemyTrait>,
}

#[derive(Component)]
pub enum EnemyType {
    Green,
    Toxic,
    Charger,
}

#[derive(Component, Clone)]
pub enum EnemyTrait {
    LeaveTrail {
        timer: Timer,
        trail_damage: f32,
    },
    Charge {
        state: ChargeState,
        charge_timer: Timer,
        charge_distance: f32,
        charge_speed: f32,
        target_position: Option<Vec2>,
    },
}

#[derive(Clone)]
pub enum ChargeState {
    Approaching,
    Preparing,
    Charging,
    Cooldown,
}


#[derive(Component)]
pub struct Trail {
    pub damage: f32,
    pub lifetime: Timer,
    pub radius: f32,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_enemies.run_if(on_timer(Duration::from_secs_f32(ENEMY_SPAWN_INTERVAL))),
                update_enemy_transform,
                apply_enemy_traits,
                despawn_dead_enemies,
                update_trails,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn despawn_dead_enemies(
    mut commands: Commands,
    enemy_query: Query<(&Enemy, Entity), With<Enemy>>,
    mut wave: ResMut<Wave>,
) {
    if enemy_query.is_empty() {
        return;
    }

    for (enemy, entity) in enemy_query.iter() {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn();
            wave.enemies_left -= 1;
        }
    }
}

fn update_enemy_transform(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&Enemy, &mut Transform), Without<Player>>,
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation;
    for (enemy, mut transform) in enemy_query.iter_mut() {
        if !enemy.traits.iter().any(|t| matches!(t, EnemyTrait::Charge { .. })) {
            let dir = (player_pos - transform.translation).normalize();
            transform.translation += dir * enemy.speed;
        }
    }
}

fn spawn_enemies(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    player_query: Query<&Transform, With<Player>>,
    mut wave: ResMut<Wave>,
) {
    if wave.enemies_left == 0 {
        let wave_count = calculate_enemies_per_wave(wave.number);
        wave.number += 1;
        wave.enemies_left = wave_count;
        wave.enemies_total = wave_count;
        wave.enemies_spawned = 0;
    }

    if wave.enemies_spawned >= wave.enemies_total || player_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation.truncate();
    for _ in 0..wave.enemies_left.min(SPAWN_RATE_PER_SECOND as u32) {
        let (x, y) = get_random_position_around(player_pos);
        let enemy_type = EnemyType::get_rand_enemy();
        let enemy_config = enemy_type.get_config();
        
        commands.spawn((
            SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_translation(vec3(x, y, 1.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: enemy_type.get_base_sprite_index(),
            },
            Enemy {
                health: enemy_config.health,
                speed: enemy_config.speed,
                damage: enemy_config.damage,
                traits: enemy_config.traits,
            },
            enemy_type,
            AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
            GameEntity,
        ));
        wave.enemies_spawned += 1;
    }
}

fn get_random_position_around(pos: Vec2) -> (f32, f32) {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..PI * 2.0);
    let dist = rng.gen_range(1000.0..3000.0);

    let offset_x = angle.cos() * dist;
    let offset_y = angle.sin() * dist;

    let random_x = pos.x + offset_x;
    let random_y = pos.y + offset_y;

    (
        random_x.clamp(-WORLD_W, WORLD_W),
        random_y.clamp(-WORLD_H, WORLD_H),
    )
}

fn calculate_enemies_per_wave(wave: u32) -> u32 {
    10 * 2_u32.pow(wave)
}

fn update_trails(
    mut commands: Commands,
    time: Res<Time>,
    mut trail_query: Query<(Entity, &mut Trail)>,
) {
    for (entity, mut trail) in trail_query.iter_mut() {
        trail.lifetime.tick(time.delta());
        if trail.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn apply_enemy_traits(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_query: Query<(&mut Enemy, &mut Transform, &mut TextureAtlas)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_pos = player_query.single().translation;

    for (mut enemy, mut transform, mut texture_atlas) in enemy_query.iter_mut() {
        let enemy_speed = enemy.speed; 
        let mut movement = Vec3::ZERO;
        let mut is_charging = false;
        let mut charge_state = None;

        for trait_ in enemy.traits.iter_mut() {
            match trait_ {
                EnemyTrait::LeaveTrail { timer, trail_damage } => {
                    timer.tick(time.delta());
                    if timer.just_finished() {
                        spawn_trail(&mut commands, transform.translation, *trail_damage);
                    }
                }
                EnemyTrait::Charge { state, charge_timer, charge_distance, charge_speed, target_position } => {
                    is_charging = true;
                    charge_state = Some((state, charge_timer, *charge_distance, *charge_speed, target_position));
                }
            }
        }

        if let Some((state, charge_timer, charge_distance, charge_speed, target_position)) = charge_state {
            match state {
                ChargeState::Approaching => {
                    let dir = (player_pos.xy() - transform.translation.xy()).normalize();
                    movement = dir.extend(0.0) * enemy_speed;
                    if transform.translation.xy().distance(player_pos.xy()) <= charge_distance {
                        *state = ChargeState::Preparing;
                        charge_timer.reset();
                    }
                }
                ChargeState::Preparing => {
                    texture_atlas.index = 17;
                    charge_timer.tick(time.delta());
                    if charge_timer.just_finished() {
                        *state = ChargeState::Charging;
                        *target_position = Some(player_pos.xy());
                    }
                }
                ChargeState::Charging => {
                    if let Some(target) = *target_position {
                        let dir = (target - transform.translation.xy()).normalize();
                        movement = dir.extend(0.0) * charge_speed;
                        if transform.translation.xy().distance(target) < 10.0 {
                            *state = ChargeState::Cooldown;
                            charge_timer.reset();
                        }
                    }
                }
                ChargeState::Cooldown => {
                    texture_atlas.index = 17;
                    charge_timer.tick(time.delta());
                    if charge_timer.just_finished() {
                        *state = ChargeState::Approaching;
                    }
                }
            }
        }

        if !is_charging {
            let dir = (player_pos - transform.translation).normalize();
            movement = dir * enemy_speed;
        }

        transform.translation += movement;
    }
}


fn spawn_trail(commands: &mut Commands, position: Vec3, damage: f32) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.0, 0.8, 0.0, 0.5), 
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        },
        Trail {
            damage,
            lifetime: Timer::from_seconds(5.0, TimerMode::Once),
            radius: 10.0,
        },
        GameEntity,
    ));
}

impl EnemyType {
    fn get_rand_enemy() -> Self {
        let mut rng = rand::thread_rng();
        let rand_index = rng.gen_range(0..5);
        match rand_index {
            0 => Self::Green,
            1 => Self::Charger,
            _ => Self::Toxic,
        }
    }

    pub fn get_base_sprite_index(&self) -> usize {
        match self {
            EnemyType::Green => 8,
            EnemyType::Toxic => 12,
            EnemyType::Charger => 20,
        }
    }

    fn get_config(&self) -> EnemyConfig {
        match self {
            EnemyType::Green => EnemyConfig {
                health: 100.0,
                speed: 1.0,
                damage: 1.0,
                traits: vec![],
            },
            EnemyType::Toxic => EnemyConfig {
                health: 50.0,
                speed: 2.0,
                damage: 0.0,
                traits: vec![EnemyTrait::LeaveTrail {
                    timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                    trail_damage: 2.0,
                }],
            },
            EnemyType::Charger => EnemyConfig {
                health: 80.0,
                speed: 4.0,
                damage: 8.0,
                traits: vec![EnemyTrait::Charge {
                    state: ChargeState::Approaching,
                    charge_timer: Timer::from_seconds(1.5, TimerMode::Once),
                    charge_distance: 200.0,
                    charge_speed: 15.0,
                    target_position: None,
                }],
            }
        }
    }
}

struct EnemyConfig {
    health: f32,
    speed: f32,
    damage: f32,
    traits: Vec<EnemyTrait>,
}