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
struct PauseMenuRoot;
#[derive(Component)]
enum MenuButton {
    Resume,
    Restart,
    Quit,
}

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
            (
                setup_ui,
                setup_pause_menu,
                setup_health_bar.after(init_world),
            ),
        )
        .add_systems(Update, menu_navigation.run_if(in_state(GameState::Paused)))
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
            (toggle_loot_ui_visibility.run_if(
                in_state(GameState::Combat)
                    .or(in_state(GameState::Ui))
                    .or(in_state(GameState::Paused)),
            ),),
        )
        .add_systems(OnEnter(GameState::Ui), update_ui)
        .add_systems(
            Update,
            update_wave_display.run_if(in_state(GameState::Combat).or(in_state(GameState::Paused))),
        )
        .add_systems(
            Update,
            (handle_pause_input, update_health_bar)
                .run_if(in_state(GameState::Combat).or(in_state(GameState::Paused))),
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
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            GlobalZIndex(3),
            Visibility::Hidden,
            BackgroundColor(Color::linear_rgb(0.6, 0.2, 0.2)),
            InGameEntity,
        ))
        .with_children(|parent| {
            // Left side: Loot information
            parent
                .spawn(Node {
                    width: Val::Percent(30.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(2.0)),

                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Loot Information"),
                        TextFont {
                            font: font.0.clone(),
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        LootInfoText,
                    ));
                });

            // Right side: Player information
            parent
                .spawn((
                    Node {
                        width: Val::Percent(70.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexEnd,
                        padding: UiRect::all(Val::Px(10.0)),
                        border: UiRect::left(Val::Px(6.0)),
                        ..default()
                    },
                    BorderColor(Color::linear_rgb(0.0, 0.0, 0.0)),
                ))
                .with_children(|parent| {
                    // Health
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                ImageNode {
                                    image: health_icon.clone(),

                                    ..default()
                                },
                                Node {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    margin: UiRect::right(Val::Px(10.0)),
                                    ..default()
                                },
                            ));
                            parent.spawn((
                                Text::new("Health: "),
                                TextFont {
                                    font: font.0.clone(),
                                    font_size: 30.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                                PlayerHealthText,
                            ));
                        });

                    // Level
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                ImageNode {
                                    image: level_icon.clone(),

                                    ..default()
                                },
                                Node {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    margin: UiRect::right(Val::Px(10.0)),
                                    ..default()
                                },
                            ));
                            parent.spawn((
                                Text::new("Level: "),
                                TextFont {
                                    font: font.0.clone(),
                                    font_size: 30.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                                PlayerLevelText,
                            ));
                        });

                    // XP
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,

                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                ImageNode {
                                    image: xp_icon.clone(),

                                    ..default()
                                },
                                Node {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    margin: UiRect::right(Val::Px(10.0)),
                                    ..default()
                                },
                            ));
                            parent.spawn((
                                Text::new("XP: "),
                                TextFont {
                                    font: font.0.clone(),
                                    font_size: 30.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                                PlayerXpText,
                            ));
                        });

                    // Defense
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            margin: UiRect::bottom(Val::Px(10.0)),

                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                ImageNode {
                                    image: defense_icon.clone(),

                                    ..default()
                                },
                                Node {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    margin: UiRect::right(Val::Px(10.0)),
                                    ..default()
                                },
                            ));
                            parent.spawn((
                                Text::new("Defense: "),
                                TextFont {
                                    font: font.0.clone(),
                                    font_size: 30.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
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
            *health_text = format!("Health: {}", health.0).into();
        }
        if let Ok(mut defense_text) = param_set.p3().get_single_mut() {
            *defense_text = format!("Defense: {}", defense.0).into();
        }
    }

    if let Ok(mut level_text) = param_set.p1().get_single_mut() {
        *level_text = format!("Level: {}", level.level()).into();
    }

    if let Ok(mut xp_text) = param_set.p2().get_single_mut() {
        *xp_text = format!("XP: {}/{}", level.current_xp(), level.xp_threshold()).into();
    }

    if let Ok(player_inventory) = player_inventory_query.get_single() {
        if let Ok(mut loot_text) = param_set.p4().get_single_mut() {
            *loot_text = format!(
                "Health Potions: {}\nSpeed Potions: {}\nGuns: {}\nArmors: {}",
                player_inventory.health_potions.len(),
                player_inventory.speed_potions.len(),
                player_inventory.guns.len(),
                player_inventory.armors.len()
            )
            .into();
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
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Play"),
                        TextFont {
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::BLACK),
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
    mut query: Query<&mut Visibility, With<PauseMenuRoot>>,
    current_state: Res<State<GameState>>,
) {
    if action_state.just_pressed(&Action::TogglePause) {
        let mut visibility = query.single_mut();
        if current_state.get() == &GameState::Combat {
            next_state.set(GameState::Paused);

            *visibility = Visibility::Visible;
        }
    }
}

fn setup_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>, font: Res<UiFont>) {
    let _ = asset_server;
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Visibility::Hidden,
            PauseMenuRoot,
            InGameEntity,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(200.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::linear_rgb(0.2, 0.2, 0.2)),
                ))
                .with_children(|parent| {
                    // Resume button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::linear_rgb(0.8, 0.8, 0.8)),
                            MenuButton::Resume,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Resume"),
                                TextFont {
                                    font: font.0.clone(),
                                    font_size: 30.0,
                                    ..default()
                                },
                                TextColor(Color::BLACK),
                            ));
                        });

                    // Restart button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::linear_rgb(0.8, 0.8, 0.8)),
                            MenuButton::Restart,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Restart"),
                                TextFont {
                                    font: font.0.clone(),
                                    font_size: 30.0,
                                    ..default()
                                },
                                TextColor(Color::BLACK),
                            ));
                        });

                    // Quit button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::linear_rgb(0.8, 0.8, 0.8)),
                            MenuButton::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Quit"),
                                TextFont {
                                    font: font.0.clone(),
                                    font_size: 30.0,
                                    ..default()
                                },
                                TextColor(Color::BLACK),
                            ));
                        });
                });
        });
}

fn menu_navigation(
    mut next_state: ResMut<NextState<GameState>>,
    action_state: Res<ActionState<Action>>,
    all_entities: Query<Entity, With<InGameEntity>>,
    mut visibility_query: Query<&mut Visibility, With<PauseMenuRoot>>,
    mut selected_button: Local<usize>,
    mut query: Query<(&MenuButton, &mut BackgroundColor)>,
    commands: Commands,
    mut exit: EventWriter<AppExit>,
) {
    let button_count = query.iter().count();

    if action_state.just_pressed(&Action::NavigateUp) {
        *selected_button = (*selected_button + button_count - 1) % button_count;
    }

    if action_state.just_pressed(&Action::NavigateDown) {
        *selected_button = (*selected_button + 1) % button_count;
    }

    for (i, (_, mut color)) in query.iter_mut().enumerate() {
        if i == *selected_button {
            *color = BackgroundColor(Color::linear_rgb(0.5, 0.5, 0.5));
        } else {
            *color = BackgroundColor(Color::linear_rgb(0.8, 0.8, 0.8));
        }
    }

    if action_state.just_pressed(&Action::Confirm) {
        let mut visibility = visibility_query.single_mut();

        if let Some((button, _)) = query.iter().nth(*selected_button) {
            match button {
                MenuButton::Resume => {
                    next_state.set(GameState::Combat);
                    *visibility = Visibility::Hidden;
                }
                MenuButton::Restart => {
                    next_state.set(GameState::Initializing);
                    cleanup_entities(commands, all_entities);
                }
                MenuButton::Quit => {
                    exit.send(AppExit::Success);
                }
            }
            *selected_button = 0;
        }
    }
}

fn setup_health_bar(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).with_children(|parent| {
            // Health bar background
            parent.spawn((
                Sprite {
                    color: Color::linear_rgb(0.5, 0.5, 0.5),
                    custom_size: Some(Vec2::new(18.0, 4.0)), // Background size
                    ..default()
                },
                Transform {
                    translation: Vec3::new(0.0, 16.0, 0.0), // Position above the player
                    ..default()
                },
            ));

            // Health bar fill
            parent
                .spawn((
                    Sprite {
                        color: Color::linear_rgb(0.0, 1.0, 0.0),
                        custom_size: Some(Vec2::new(18.0, 4.0)), // Fill size, will be updated dynamically
                        ..default()
                    },
                    Transform {
                        translation: Vec3::new(0.0, 16.0, 1.0), // Position above the player, slightly in front
                        ..default()
                    },
                ))
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
            transform.translation.x = -9.0 + (9.0 * health_percentage);
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
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Percent(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            InGameEntity,
            WaveDisplayRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Wave 1"),
                TextFont {
                    font: font.0.clone(),
                    font_size: 40.0,
                    ..default()
                },
                TextColor::WHITE,
                WaveDisplay,
            ));
        });
}

fn update_wave_display(mut wave_query: Query<&mut Text, With<WaveDisplay>>, wave: Res<Wave>) {
    if wave.is_changed() {
        if let Ok(mut text) = wave_query.get_single_mut() {
            *text = Text::from(format!("Wave {}", wave.number));
        }
    }
}
