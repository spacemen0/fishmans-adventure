use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    input::Action,
    player::{Defense, Health, Player, PlayerInventory},
    resources::{Level, UiFont},
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
                    update_ui.run_if(in_state(GameState::Combat)),
                    toggle_ui_visibility
                        .run_if(in_state(GameState::Combat).or_else(in_state(GameState::Paused))),
                ),
            );
    }
}

fn setup_ui(mut commands: Commands, font: Res<UiFont>, asset_server: Res<AssetServer>) {
    // Load icons or images
    let health_icon = asset_server.load("icons/health.png");
    let level_icon = asset_server.load("icons/level.png");
    let xp_icon = asset_server.load("icons/xp.png");
    let defense_icon = asset_server.load("icons/defense.png");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                background_color: BackgroundColor(Color::linear_rgb(0.6, 0.2, 0.2)),
                visibility: Visibility::Hidden,
                z_index: ZIndex::Global(3),
                ..default()
            },
            InGameEntity,
        ))
        .with_children(|parent| {
            // Left side: Loot information
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(30.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(10.0)),
                        border: UiRect::all(Val::Px(2.0)),
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
                        align_items: AlignItems::FlexEnd,
                        padding: UiRect::all(Val::Px(10.0)),
                        border: UiRect::left(Val::Px(6.0)),
                        ..default()
                    },
                    border_color: BorderColor(Color::linear_rgb(0.0, 0.0, 0.0)),
                    ..default()
                })
                .with_children(|parent| {
                    // Health
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                image: health_icon.clone().into(),
                                style: Style {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    margin: UiRect::right(Val::Px(10.0)),
                                    ..default()
                                },
                                ..default()
                            });
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
                        });

                    // Level
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                image: level_icon.clone().into(),
                                style: Style {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    margin: UiRect::right(Val::Px(10.0)),
                                    ..default()
                                },
                                ..default()
                            });
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
                        });

                    // XP
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                image: xp_icon.clone().into(),
                                style: Style {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    margin: UiRect::right(Val::Px(10.0)),
                                    ..default()
                                },
                                ..default()
                            });
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
                        });

                    // Defense
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                image: defense_icon.clone().into(),
                                style: Style {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    margin: UiRect::right(Val::Px(10.0)),
                                    ..default()
                                },
                                ..default()
                            });
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
                });
        })
        .insert(UiRoot);
}

fn update_ui(
    level: Res<Level>,
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

    if let Ok(mut level_text) = param_set.p1().get_single_mut() {
        level_text.sections[0].value = format!("Level: {}", level.level());
    }

    if let Ok(mut xp_text) = param_set.p2().get_single_mut() {
        xp_text.sections[0].value = format!("XP: {}/{}", level.current_xp(), level.xp_threshold());
    }

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
