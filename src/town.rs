// use crate::{
//     input::Action, player::Player, resources::GlobalTextureAtlas, state::GameState,
//     utils::InGameEntity,
// };
// use bevy::prelude::*;
// use leafwing_input_manager::prelude::ActionState;

// pub struct TownPlugin;

// impl Plugin for TownPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(OnEnter(GameState::Town), setup_town)
//             .add_systems(Update, town_systems.run_if(in_state(GameState::Town)))
//             .add_systems(OnExit(GameState::Town), cleanup_town);
//     }
// }

// fn setup_town(
//     mut commands: Commands,
//     mut player_query: Query<&mut Transform, With<Player>>,
//     _handle: Res<GlobalTextureAtlas>,
// ) {
//     // Spawn town buildings, NPCs, etc.
//     commands.spawn((
//         SpriteBundle {
//             sprite: Sprite {
//                 color: Color::srgb(0.5, 0.5, 0.8),
//                 custom_size: Some(Vec2::new(50.0, 50.0)),
//                 ..default()
//             },
//             transform: Transform::from_xyz(0.0, 0.0, 0.0),
//             ..default()
//         },
//         InGameEntity,
//     ));

//     // Spawn town exit portal
//     commands.spawn((
//         SpriteBundle {
//             sprite: Sprite {
//                 color: Color::srgba(1.0, 0.0, 0.0, 0.5),
//                 custom_size: Some(Vec2::new(50.0, 80.0)),
//                 ..default()
//             },
//             transform: Transform::from_xyz(200.0, 0.0, 0.0),
//             ..default()
//         },
//         TownPortal,
//         InGameEntity,
//     ));

//     // Spawn NPCs
//     commands.spawn((
//         SpriteBundle {
//             sprite: Sprite {
//                 color: Color::srgb(0.0, 1.0, 0.0),
//                 custom_size: Some(Vec2::new(30.0, 30.0)),
//                 ..default()
//             },
//             transform: Transform::from_xyz(-100.0, 0.0, 0.0),
//             ..default()
//         },
//         NPC,
//         InGameEntity,
//     ));

//     // Ensure the player entity is properly initialized in the town scene
//     if let Ok(mut player_transform) = player_query.get_single_mut() {
//         player_transform.translation = Vec3::new(0.0, 0.0, 10.0); // Set the player's position in the town
//     }
// }

// #[derive(Component)]
// pub struct TownPortal;

// #[derive(Component)]
// pub struct NPC;

// fn town_systems(
//     mut next_state: ResMut<NextState<GameState>>,
//     player_query: Query<&Transform, With<Player>>,
//     portal_query: Query<&Transform, With<TownPortal>>,
//     npc_query: Query<&Transform, With<NPC>>,
//     active_state: Res<ActionState<Action>>,
// ) {
//     if let Ok(player_transform) = player_query.get_single() {
//         for portal_transform in portal_query.iter() {
//             let distance = player_transform
//                 .translation
//                 .distance(portal_transform.translation);

//             if distance < 60.0 && active_state.just_pressed(&Action::Interact) {
//                 next_state.set(GameState::Combat);
//             }
//         }

//         for npc_transform in npc_query.iter() {
//             let distance = player_transform
//                 .translation
//                 .distance(npc_transform.translation);

//             if distance < 60.0 && active_state.just_pressed(&Action::Interact) {
//                 // Handle NPC interaction
//                 println!("Interacting with NPC");
//             }
//         }
//     }
// }

// fn cleanup_town(mut commands: Commands, query: Query<Entity, With<InGameEntity>>) {
//     for entity in query.iter() {
//         commands.entity(entity).despawn_recursive();
//     }
// }
