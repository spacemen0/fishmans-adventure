use bevy::prelude::*;

use crate::{
    enemy::Enemy,
    enemy::EnemyType,
    gun::Gun,
    player::{Player, PlayerInventory, PlayerState},
    state::GameState,
    CursorPosition,
};

pub struct AnimationPlugin;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                animation_timer_tick,
                animate_player,
                animate_enemy,
                flip_gun_sprite_y,
                flip_player_sprite_x,
                flip_enemy_sprite_x,
            )
                .run_if(in_state(GameState::InGame).or_else(in_state(GameState::Paused))),
        );
    }
}

fn animation_timer_tick(
    time: Res<Time>,
    mut query: Query<&mut AnimationTimer, With<AnimationTimer>>,
) {
    for mut timer in query.iter_mut() {
        timer.tick(time.delta());
    }
}

fn animate_player(
    mut player_query: Query<(&mut TextureAtlas, &PlayerState, &AnimationTimer), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut atlas, state, timer) = player_query.single_mut();
    if timer.just_finished() {
        let base_sprite_index = match state {
            PlayerState::Idle => 0,
            PlayerState::Run => 4,
        };
        atlas.index = base_sprite_index + (atlas.index + 1) % 4;
    }
}

fn animate_enemy(
    mut enemy_query: Query<(&mut TextureAtlas, &AnimationTimer, &EnemyType), With<Enemy>>,
) {
    for (mut atlas, timer, enemy_type) in enemy_query.iter_mut() {
        if timer.just_finished() {
            let config = enemy_type.get_config();
            atlas.index = config.sprite_index + (atlas.index + 1) % 4;
        }
    }
}

fn flip_player_sprite_x(
    cursor_position: Res<CursorPosition>,
    mut player_query: Query<(&mut Sprite, &Transform), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut sprite, transform) = player_query.single_mut();
    if let Some(cursor_position) = cursor_position.0 {
        if cursor_position.x > transform.translation.x {
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
        }
    }
}

fn flip_enemy_sprite_x(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Sprite, &Transform), With<Enemy>>,
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation;
    for (mut sprite, transform) in enemy_query.iter_mut() {
        if transform.translation.x < player_pos.x {
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
        }
    }
}

fn flip_gun_sprite_y(
    cursor_position: Res<CursorPosition>,
    player_query: Query<&PlayerInventory, With<Player>>,
    mut gun_query: Query<(&mut Sprite, &Transform), With<Gun>>,
) {
    // Check if player has an active gun
    if let Ok(inventory) = player_query.get_single() {
        if let Some(active_gun) = inventory.guns.get(inventory.active_gun_index) {
            // Get the sprite and transform of the active gun
            if let Ok((mut sprite, transform)) = gun_query.get_mut(*active_gun) {
                if let Some(cursor_position) = cursor_position.0 {
                    // Flip the gun sprite based on cursor position relative to gun
                    if cursor_position.x > transform.translation.x {
                        sprite.flip_y = false;
                    } else {
                        sprite.flip_y = true;
                    }
                }
            }
        }
    }
}
