use crate::{
    dialog::{ActiveDialog, DialogType, ShowDialogEvent},
    player::Player,
    resources::Wave,
    state::GameState,
    utils::InGameEntity,
};
use bevy::prelude::*;

pub struct PortalPlugin;

#[derive(Component)]
pub struct Portal;

impl Plugin for PortalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_portal, portal_interaction).run_if(in_state(GameState::Combat)),
        );
    }
}

fn spawn_portal(
    mut commands: Commands,
    mut wave: ResMut<Wave>,
    player_query: Query<&Transform, With<Player>>,
) {
    if wave.requires_portal && !wave.portal_spawned && wave.enemies_left == 0 {
        if let Ok(player_transform) = player_query.get_single() {
            let portal_position = player_transform.translation + Vec3::new(100.0, 0.0, 0.0);

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.5, 0.0, 1.0, 0.5),
                        custom_size: Some(Vec2::new(50.0, 80.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(portal_position),
                    ..default()
                },
                Portal,
                InGameEntity,
            ));

            wave.portal_spawned = true;
        }
    }
}

fn portal_interaction(
    player_query: Query<&Transform, With<Player>>,
    portal_query: Query<&Transform, With<Portal>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ev_show_dialog: EventWriter<ShowDialogEvent>,
    active_dialog: Res<ActiveDialog>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for portal_transform in portal_query.iter() {
            let distance = player_transform
                .translation
                .distance(portal_transform.translation);

            if distance < 60.0
                && keyboard_input.just_pressed(KeyCode::KeyX)
                && active_dialog.0.is_none()
            {
                ev_show_dialog.send(ShowDialogEvent(DialogType::Portal));
                next_state.set(GameState::Town);
            }
        }
    }
}
