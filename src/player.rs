use std::time::Duration;

use armor::{Armor, ArmorStats};
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use enemy::Collider;
use gun::HasLifespan;
use utils::{calculate_defense_increase, calculate_health_increase, safe_subtract};
use world::InGameEntity;

use crate::state::GameState;
use crate::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Health(pub u32);
#[derive(Component)]
pub struct Speed(pub u32);
#[derive(Component)]
pub struct Defense(pub u32);
#[derive(Component)]
pub struct PlayerInventory {
    pub guns: Vec<Entity>,
    pub health_potions: Vec<Entity>,
    pub speed_potions: Vec<Entity>,
    pub armors: Vec<Entity>,
    pub active_gun_index: usize,
    pub active_armor_index: usize,
}

#[derive(Component)]
pub struct InvincibilityEffect(pub Stopwatch, pub f32);
#[derive(Component)]
pub struct AccelerationEffect(pub Stopwatch, pub f32, pub u32);

#[derive(Component, Default, Debug)]
pub enum PlayerState {
    #[default]
    Idle,
    Run,
}

#[derive(Event)]
pub struct PlayerDamagedEvent {
    pub damage: u32,
}
#[derive(Event)]
pub struct PlayerLevelingUpEvent {
    pub new_level: u32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDamagedEvent>()
            .add_event::<PlayerLevelingUpEvent>()
            .add_systems(
                Update,
                (
                    handle_player_death,
                    handle_player_input,
                    handle_player_damaged_events,
                    handle_invincibility_effect,
                    handle_acceleration_effect,
                    handle_leveling_up,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

fn handle_player_damaged_events(
    mut commands: Commands,
    mut player_query: Query<
        (
            &mut Health,
            &PlayerState,
            &Defense,
            &mut PlayerInventory,
            &Transform,
        ),
        With<Player>,
    >,
    mut armor_query: Query<(&mut ArmorStats, Entity), With<Armor>>,
    mut events: EventReader<PlayerDamagedEvent>,
    asset_server: Res<AssetServer>,
) {
    if player_query.is_empty() {
        return;
    }
    let (mut health, _player_state, player_defense, mut inventory, player_transform) =
        player_query.single_mut();

    for event in events.read() {
        if health.0 > 0 {
            let mut total_defense = player_defense.0;

            if let Some(active_armor_entity) = inventory.armors.get(inventory.active_armor_index) {
                if let Ok((mut armor_stats, armor_entity)) =
                    armor_query.get_mut(*active_armor_entity)
                {
                    total_defense += armor_stats.defense;

                    let damage_after_defense = safe_subtract(event.damage, total_defense);
                    health.0 = safe_subtract(health.0, damage_after_defense);

                    armor_stats.durability =
                        safe_subtract(armor_stats.durability, damage_after_defense);

                    if armor_stats.durability <= 0 {
                        commands.entity(armor_entity).despawn();
                        let armor_to_remove = inventory.active_armor_index;
                        inventory.armors.remove(armor_to_remove);
                        if inventory.active_armor_index >= inventory.armors.len() {
                            inventory.active_armor_index = 0;
                        }
                    }
                    if damage_after_defense > 0 {
                        spawn_damage_text(
                            &mut commands,
                            &asset_server,
                            player_transform.translation,
                            damage_after_defense,
                        );
                    }
                }
            } else {
                let damage_after_defense = safe_subtract(event.damage, player_defense.0);
                health.0 = safe_subtract(health.0, damage_after_defense);
                if damage_after_defense > 0 {
                    spawn_damage_text(
                        &mut commands,
                        &asset_server,
                        player_transform.translation,
                        damage_after_defense,
                    );
                }
            }
        }
    }
}

fn handle_leveling_up(
    mut event_reader: EventReader<PlayerLevelingUpEvent>,
    mut player_query: Query<(&mut Health, &mut Defense), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut health, mut defense) = player_query.single_mut();

    for event in event_reader.read() {
        let level = event.new_level as u32;
        health.0 += calculate_health_increase(level);
        defense.0 += calculate_defense_increase(level);
    }
}

fn spawn_damage_text(
    commands: &mut Commands,
    asset_server: &AssetServer,
    position: Vec3,
    damage: u32,
) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("-{}", damage),
                TextStyle {
                    font: asset_server.load("monogram.ttf"),
                    font_size: 50.0,
                    color: Color::srgb(1.0, 0.0, 0.0),
                },
            ),
            transform: Transform {
                translation: position + Vec3::new(0.0, 50.0, 0.0),
                ..default()
            },
            ..default()
        },
        Collider { radius: 5 },
        HasLifespan::new(Duration::from_secs(1)),
        InGameEntity,
    ));
}

fn handle_player_death(
    player_query: Query<(&Health, Entity), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if player_query.is_empty() {
        return;
    }
    let player = player_query.single();
    if player.0 .0 == 0 {
        next_state.set(GameState::MainMenu);
    }
}

fn handle_invincibility_effect(
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(&mut InvincibilityEffect, Entity), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }
    let (mut invincibility_effect, entity) = player_query.single_mut();

    if invincibility_effect.0.elapsed_secs() >= invincibility_effect.1 {
        commands.entity(entity).remove::<InvincibilityEffect>();
    }
    invincibility_effect.0.tick(time.delta());
}
fn handle_acceleration_effect(
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(&mut AccelerationEffect, &mut Speed, Entity), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }
    let (mut acceleration_effect, mut speed, entity) = player_query.single_mut();

    if acceleration_effect.0.elapsed_secs() >= acceleration_effect.1 {
        commands.entity(entity).remove::<AccelerationEffect>();
        speed.0 -= acceleration_effect.2;
    }
    acceleration_effect.0.tick(time.delta());
}

pub fn handle_player_input(
    mut player_query: Query<(&mut Transform, &mut PlayerState, &Speed), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut transform, mut player_state, speed) = player_query.single_mut();
    let w_key = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    let a_key = keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
    let s_key = keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
    let d_key =
        keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);

    let mut delta = Vec2::ZERO;
    if w_key {
        delta.y += 1.0;
    }
    if s_key {
        delta.y -= 1.0;
    }
    if a_key {
        delta.x -= 1.0;
    }
    if d_key {
        delta.x += 1.0;
    }
    delta = delta.normalize();

    if delta.is_finite() && (w_key || a_key || s_key || d_key) {
        let desired_position = transform.translation.xy() + delta * speed.0 as f32;
        let clamped_x = desired_position.x.clamp(-WORLD_W, WORLD_W);
        let clamped_y = desired_position.y.clamp(-WORLD_H, WORLD_H);
        transform.translation = vec3(clamped_x, clamped_y, transform.translation.z);

        transform.translation.z = 10.0;
        *player_state = PlayerState::Run;
    } else {
        *player_state = PlayerState::Idle;
    }
}
