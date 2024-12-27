use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    input::Action,
    player::{Defense, Health, Player, PlayerInventory},
    resources::UiFont,
    state::GameState,
    utils::InGameEntity,
};

pub struct UiPlugin;

#[derive(Component)]
struct PlayerHealthText;

#[derive(Component)]
struct PlayerLevelText;

#[derive(Component)]
struct PlayerXpText;

#[derive(Component)]
struct PlayerDefenseText;

#[derive(Component)]
struct LootInfoText;

#[derive(Component)]
struct UiRoot;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Initializing), setup_ui)
            .add_systems(
                Update,
                (
                    update_ui,
                    toggle_ui_visibility.run_if(in_state(GameState::Combat)),
                ),
            );
    }
}

fn setup_ui(mut commands: Commands, font: Res<UiFont>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                background_color: BackgroundColor(Color::linear_rgb(1.0, 2.0, 1.0)),
                ..default()
            },
            InGameEntity,
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            // Left side: Loot information
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(30.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Loot Information",
                            TextStyle {
                                font: font.0.clone(),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        LootInfoText,
                    ));
                });

            // Right side: Player information
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(70.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Health: ",
                            TextStyle {
                                font: font.0.clone(),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        PlayerHealthText,
                    ));
                    parent.spawn((
                        TextBundle::from_section(
                            "Level: ",
                            TextStyle {
                                font: font.0.clone(),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        PlayerLevelText,
                    ));
                    parent.spawn((
                        TextBundle::from_section(
                            "XP: ",
                            TextStyle {
                                font: font.0.clone(),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        PlayerXpText,
                    ));
                    parent.spawn((
                        TextBundle::from_section(
                            "Defense: ",
                            TextStyle {
                                font: font.0.clone(),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        PlayerDefenseText,
                    ));
                });
        })
        .insert(UiRoot);
}

fn update_ui(
    player_query: Query<(&Health, &Defense), With<Player>>,
    mut param_set: ParamSet<(
        Query<&mut Text, With<PlayerHealthText>>,
        Query<&mut Text, With<PlayerLevelText>>,
        Query<&mut Text, With<PlayerXpText>>,
        Query<&mut Text, With<PlayerDefenseText>>,
        Query<&mut Text, With<LootInfoText>>,
    )>,
    player_inventory_query: Query<&PlayerInventory, With<Player>>,
) {
    if let Ok((health, defense)) = player_query.get_single() {
        if let Ok(mut health_text) = param_set.p0().get_single_mut() {
            health_text.sections[0].value = format!("Health: {}", health.0);
        }
        if let Ok(mut defense_text) = param_set.p3().get_single_mut() {
            defense_text.sections[0].value = format!("Defense: {}", defense.0);
        }
    }

    // if let Ok(mut level_text) = param_set.p1().get_single_mut() {
    //     level_text.sections[0].value = format!("Level: {}", level.level());
    // }

    // if let Ok(mut xp_text) = param_set.p2().get_single_mut() {
    //     xp_text.sections[0].value = format!("XP: {}/{}", level.current_xp(), level.xp_threshold());
    // }

    if let Ok(player_inventory) = player_inventory_query.get_single() {
        if let Ok(mut loot_text) = param_set.p4().get_single_mut() {
            loot_text.sections[0].value = format!(
                "Health Potions: {}\nSpeed Potions: {}\nGuns: {}\nArmors: {}",
                player_inventory.health_potions.len(),
                player_inventory.speed_potions.len(),
                player_inventory.guns.len(),
                player_inventory.armors.len()
            );
        }
    }
}

fn toggle_ui_visibility(
    action_state: Res<ActionState<Action>>,
    mut ui_query: Query<&mut Visibility, With<UiRoot>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if action_state.just_pressed(&Action::ToggleLootBoard) {
        for mut visibility in ui_query.iter_mut() {
            if *visibility == Visibility::Hidden {
                *visibility = Visibility::Visible;
                next_state.set(GameState::Paused);
            } else {
                *visibility = Visibility::Hidden;
                next_state.set(GameState::Combat);
            }
        }
    }
}
