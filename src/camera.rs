use bevy::{math::vec3, prelude::*};
use bevy_pancam::{DirectionKeys, PanCam, PanCamPlugin};

use crate::{
    configs::{WH, WW},
    player::Player,
    state::GameState,
};

pub struct FollowCameraPlugin;

impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin)
            .add_systems(OnEnter(GameState::Loading), setup_camera)
            .add_systems(
                Update,
                camera_follow_player
                    .run_if(in_state(GameState::Combat).or_else(in_state(GameState::Paused))),
            );
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(PanCam {
        grab_buttons: vec![],
        move_keys: DirectionKeys::NONE,
        zoom_to_cursor: false,
        min_scale: 1.0,
        max_scale: 1.0,
        ..default()
    });
}

fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if camera_query.is_empty() || player_query.is_empty() {
        return;
    }

    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single().translation;
    let (x, y) = (player_transform.x, player_transform.y);

    camera_transform.translation = camera_transform.translation.lerp(vec3(x, y, 0.0), 0.1);
    camera_transform.translation.x = camera_transform
        .translation
        .x
        .clamp((-WW / 2.0) - 25.0, (WW / 2.0) + 25.0);
    camera_transform.translation.y = camera_transform
        .translation
        .y
        .clamp((-WH / 2.0) - 25.0, (WH / 2.0) + 25.0);
}
