use bevy::math::vec3;
use bevy::prelude::*;
use bevy::time::Stopwatch;

use crate::state::GameState;
use crate::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Health(pub f32);
#[derive(Component)]
pub struct Speed(pub f32);
#[derive(Component)]
pub struct PlayerInventory {
    pub guns: Vec<Entity>,
    pub health_potions: Vec<Entity>,
    pub speed_potions: Vec<Entity>,
    pub active_gun_index: usize,
}

#[derive(Component)]
pub struct InvincibilityEffect(pub Stopwatch, pub f32);
#[derive(Component)]
pub struct AccelerationEffect(pub Stopwatch, pub f32, pub f32);

#[derive(Component, Default, Debug)]
pub enum PlayerState {
    #[default]
    Idle,
    Run,
}

#[derive(Event)]
pub struct PlayerEnemyCollisionEvent {
    pub damage: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerEnemyCollisionEvent>().add_systems(
            Update,
            (
                handle_player_death,
                handle_player_input,
                handle_player_enemy_collision_events,
                handle_invincibility_effect,
                handle_acceleration_effect,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn handle_player_enemy_collision_events(
    mut player_query: Query<(&mut Health, &PlayerState), With<Player>>,
    mut events: EventReader<PlayerEnemyCollisionEvent>,
) {
    if player_query.is_empty() {
        return;
    }
    let (mut health, _player_state) = player_query.single_mut();
    for event in events.read() {
        if health.0 > 0.0 {
            health.0 = (health.0 - event.damage).max(0.0);
        }
    }
}

fn handle_player_death(
    player_query: Query<(&Health, Entity), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if player_query.is_empty() {
        return;
    }
    let player = player_query.single();
    if player.0 .0 <= 0.0 {
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
        let desired_position = transform.translation.xy() + delta * speed.0;
        let clamped_x = desired_position.x.clamp(-WORLD_W, WORLD_W);
        let clamped_y = desired_position.y.clamp(-WORLD_H, WORLD_H);
        transform.translation = vec3(clamped_x, clamped_y, transform.translation.z);

        transform.translation.z = 10.0;
        *player_state = PlayerState::Run;
    } else {
        *player_state = PlayerState::Idle;
    }
}
