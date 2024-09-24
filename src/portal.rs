use crate::player::Player;
use crate::resources::Wave;
use crate::state::GameState;
use crate::utils::calculate_enemies_per_wave;
use crate::world::InGameEntity;
use bevy::prelude::*;

pub struct PortalPlugin;

#[derive(Component)]
pub struct Portal;

impl Plugin for PortalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_portal, portal_interaction).run_if(in_state(GameState::InGame)),
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
    mut commands: Commands,
    mut wave: ResMut<Wave>,
    player_query: Query<&Transform, With<Player>>,
    portal_query: Query<(Entity, &Transform), With<Portal>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (portal_entity, portal_transform) in portal_query.iter() {
            let distance = player_transform
                .translation
                .distance(portal_transform.translation);

            if distance < 60.0 && keyboard_input.just_pressed(KeyCode::KeyX) {
                wave.requires_portal = false;
                wave.portal_spawned = false;

                let wave_count = calculate_enemies_per_wave(wave.number + 1); 
                wave.enemies_total = wave_count;
                wave.enemies_left = wave_count;
                wave.enemies_spawned = 0;
                wave.number += 1;

                commands.entity(portal_entity).despawn();

                println!("portal used, next wave: {}", wave.number);
            }
        }
    }
}
