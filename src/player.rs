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
pub struct PlayerInventory {
    pub guns: Vec<Entity>,
    pub active_gun_index: usize,
}

#[derive(Component)]
pub struct InvulnerableTimer(pub Stopwatch);

#[derive(Component, Default, Debug)]
pub enum PlayerState {
    #[default]
    Idle,
    Run,
    IdleInvulnerable,
    RunInvulnerable,
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
                handle_player_invulnerable_timer,
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
        println!("{}", event.damage);
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

fn handle_player_invulnerable_timer(
    time: Res<Time>,
    mut player_query: Query<(&mut PlayerState, &mut InvulnerableTimer), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }
    let (mut player_state, mut invulnerable_timer) = player_query.single_mut();
    match player_state.as_ref() {
        PlayerState::Idle => (),
        PlayerState::IdleInvulnerable | PlayerState::RunInvulnerable => {
            if invulnerable_timer.0.elapsed_secs() >= PLAYER_INVULNERABLE_TIME {
                invulnerable_timer.0.reset();
                *player_state = match *player_state {
                    PlayerState::IdleInvulnerable => PlayerState::Idle,
                    PlayerState::RunInvulnerable => PlayerState::Run,
                    _ => unreachable!(),
                };
            }
            invulnerable_timer.0.tick(time.delta());
        }
        PlayerState::Run => (),
    }
}

fn handle_player_input(
    mut player_query: Query<(&mut Transform, &mut PlayerState), With<Player>>,

    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut transform, mut player_state) = player_query.single_mut();
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
        let desired_position = transform.translation.xy() + delta * PLAYER_SPEED;
        let clamped_x = desired_position.x.clamp(-WORLD_W, WORLD_W);
        let clamped_y = desired_position.y.clamp(-WORLD_H, WORLD_H);
        transform.translation = vec3(clamped_x, clamped_y, transform.translation.z);

        transform.translation.z = 10.0;
        *player_state = match *player_state {
            PlayerState::Idle => PlayerState::Run,
            PlayerState::Run => PlayerState::Run,
            PlayerState::IdleInvulnerable => PlayerState::RunInvulnerable,
            PlayerState::RunInvulnerable => PlayerState::RunInvulnerable,
        }
    } else {
        *player_state = match *player_state {
            PlayerState::Idle => PlayerState::Idle,
            PlayerState::Run => PlayerState::Idle,
            PlayerState::IdleInvulnerable => PlayerState::IdleInvulnerable,
            PlayerState::RunInvulnerable => PlayerState::IdleInvulnerable,
        }
    }
}
