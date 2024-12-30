use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    configs::{DEFENSE_ICON_PATH, HEALTH_ICON_PATH, LEVEL_ICON_PATH, XP_ICON_PATH},
    input::Action,
    player::{Defense, Health, Player, PlayerInventory},
    resources::{Level, UiFont, Wave},
    state::GameState,
    utils::{cleanup_entities, InGameEntity},
    world::init_world,
};

pub struct UiPlugin;

#[derive(Component)]
struct MainMenuItem;

#[derive(Component)]
struct PlayerHealthText;

#[derive(Component)]
struct PlayerLevelText;

#[derive(Component)]
struct WaveDisplay;

#[derive(Component)]
struct PlayerXpText;

#[derive(Component)]
struct PlayerDefenseText;

#[derive(Component)]
struct LootInfoText;

#[derive(Component)]
struct PlayerHealthBar;

#[derive(Component)]
struct UiRoot;

#[derive(Component)]
struct WaveDisplayRoot;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Initializing),
            (setup_ui, setup_health_bar.after(init_world)),
        )
        .add_systems(
            OnEnter(GameState::MainMenu),
            (setup_main_menu, cleanup_entities),
        )
        .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
        .add_systems(OnEnter(GameState::Combat), setup_wave_display)
        .add_systems(
            Update,
            handle_main_menu_buttons.run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            Update,
            (toggle_loot_ui_visibility
                .run_if(in_state(GameState::Combat).or_else(in_state(GameState::Ui))),),
        )
        .add_systems(OnEnter(GameState::Ui), update_ui)
        .add_systems(
            Update,
            update_wave_display.run_if(
                in_state(GameState::Combat)
                    .or_else(in_state(GameState::Paused))
                    .and_then(resource_changed::<Wave>),
            ),
        )
        .add_systems(
            Update,
            (handle_pause_input, handle_game_restart, update_health_bar)
                .run_if(in_state(GameState::Combat).or_else(in_state(GameState::Paused))),
        );
    }
}

fn setup_ui(mut commands: Commands, font: Res<UiFont>, asset_server: Res<AssetServer>) {
    let health_icon = asset_server.load(HEALTH_ICON_PATH);
    let level_icon = asset_server.load(LEVEL_ICON_PATH);
    let xp_icon = asset_server.load(XP_ICON_PATH);
    let defense_icon = asset_server.load(DEFENSE_ICON_PATH);

    commands
        .spawn((
            Name::new("Ui"),
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

fn toggle_loot_ui_visibility(
    action_state: Res<ActionState<Action>>,
    mut ui_query: Query<&mut Visibility, With<UiRoot>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if action_state.just_pressed(&Action::ToggleLootBoard) {
        for mut visibility in ui_query.iter_mut() {
            if *visibility == Visibility::Hidden {
                *visibility = Visibility::Visible;
                next_state.set(GameState::Ui);
            } else {
                *visibility = Visibility::Hidden;
                next_state.set(GameState::Combat);
            }
        }
    }
}

fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    ));
                });
        })
        .insert(MainMenuItem);
}

fn handle_main_menu_buttons(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in interaction_query.iter() {
        if interaction == &Interaction::Pressed {
            next_state.set(GameState::Initializing);
        }
    }
}

fn despawn_main_menu(mut commands: Commands, menu_items_query: Query<Entity, With<MainMenuItem>>) {
    for e in menu_items_query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn handle_pause_input(
    action_state: Res<ActionState<Action>>,
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
) {
    if action_state.just_pressed(&Action::TogglePause) {
        match current_state.get() {
            GameState::Combat => {
                next_state.set(GameState::Paused);
            }
            GameState::Paused => {
                next_state.set(GameState::Combat);
            }
            _ => {}
        }
    }
}

fn handle_game_restart(
    commands: Commands,
    action_state: Res<ActionState<Action>>,
    all_entities: Query<Entity, With<InGameEntity>>,
    mut next_state: ResMut<NextState<GameState>>, // bug still possible, maybe only allow it when paused
) {
    if action_state.just_pressed(&Action::Restart) {
        next_state.set(GameState::Initializing);
        cleanup_entities(commands, all_entities);
    }
}

fn setup_health_bar(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).with_children(|parent| {
            // Health bar background
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::linear_rgb(0.5, 0.5, 0.5),
                    custom_size: Some(Vec2::new(18.0, 4.0)), // Background size
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 16.0, 0.0), // Position above the player
                    ..default()
                },
                ..default()
            });

            // Health bar fill
            parent
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::linear_rgb(0.0, 1.0, 0.0),
                        custom_size: Some(Vec2::new(18.0, 4.0)), // Fill size, will be updated dynamically
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(0.0, 16.0, 1.0), // Position above the player, slightly in front
                        ..default()
                    },
                    ..default()
                })
                .insert(PlayerHealthBar);
        });
    }
}

fn update_health_bar(
    player_query: Query<&Health, With<Player>>,
    mut health_bar_query: Query<(&mut Transform, &mut Sprite), With<PlayerHealthBar>>,
) {
    if let Ok(health) = player_query.get_single() {
        if let Ok((mut transform, mut sprite)) = health_bar_query.get_single_mut() {
            let health_percentage = health.0 as f32 / health.1 as f32;
            sprite.custom_size = Some(Vec2::new(18.0 * health_percentage, 4.0));
            transform.translation.x = -9.0 + (9.0 * health_percentage); // Adjust position to keep it aligned
        }
    }
}

fn setup_wave_display(
    mut commands: Commands,
    font: Res<UiFont>,
    existing_displays: Query<Entity, With<WaveDisplayRoot>>,
) {
    if !existing_displays.is_empty() {
        return;
    }

    commands
        .spawn((
            Name::new("Wave"),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    left: Val::Percent(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            InGameEntity,
            WaveDisplayRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Wave 1",
                    TextStyle {
                        font: font.0.clone(),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                ),
                WaveDisplay,
            ));
        });
}

fn update_wave_display(mut wave_query: Query<&mut Text, With<WaveDisplay>>, wave: Res<Wave>) {
    println!("wave system");
    if let Ok(mut text) = wave_query.get_single_mut() {
        text.sections[0].value = format!("Wave {}", wave.number);
    }
}
