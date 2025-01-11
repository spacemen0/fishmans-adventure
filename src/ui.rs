use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    configs::{DEFENSE_ICON_PATH, HEALTH_ICON_PATH, LEVEL_ICON_PATH, XP_ICON_PATH},
    input::Action,
    loot::Description,
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
struct DescriptionTextBox;

#[derive(Component)]
enum MenuButton {
    Resume,
    Restart,
    Quit,
}

#[derive(Component)]
struct MainMenuItem;

#[derive(Component)]
struct MainMenuText;

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
struct PlayerHealthBar;

#[derive(Component)]
struct UiRoot;

#[derive(Component)]
struct WaveDisplayRoot;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
struct GridSlot {
    x: usize,
    y: usize,
    item: Option<Entity>,
}

#[derive(Component)]
struct FocusedItem;

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
            (handle_main_menu_buttons, blink_main_menu_text).run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            Update,
            (toggle_loot_ui_visibility.run_if(
                in_state(GameState::Combat)
                    .or(in_state(GameState::Ui))
                    .or(in_state(GameState::Paused)),
            ),),
        )
        .add_systems(OnEnter(GameState::Ui), (update_ui, set_up_loot_image))
        .add_systems(
            Update,
            update_wave_display.run_if(in_state(GameState::Combat).or(in_state(GameState::Paused))),
        )
        .add_systems(
            Update,
            (handle_pause_input, update_health_bar)
                .run_if(in_state(GameState::Combat).or(in_state(GameState::Paused))),
        )
        .add_systems(
            Update,
            (navigate_loot_items, highlight_focused_item).run_if(in_state(GameState::Ui)),
        );
    }
}

fn spawn_player_info_item(
    parent: &mut ChildBuilder,
    font: &Handle<Font>,
    icon: Handle<Image>,
    label: &str,
    component: impl Component,
) {
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
                    image: icon.clone(),
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
                Text::new(format!("{}: ", label)),
                TextFont {
                    font: font.clone(),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                component,
            ));
        });
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
                    spawn_slots_grid(parent, &font.0, "Health Potions", 4, 0);
                    spawn_slots_grid(parent, &font.0, "Speed Potions", 4, 1);
                    spawn_slots_grid(parent, &font.0, "Guns", 4, 2);
                    spawn_slots_grid(parent, &font.0, "Armors", 4, 3);
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
                    spawn_player_info_item(
                        parent,
                        &font.0,
                        health_icon,
                        "Health",
                        PlayerHealthText,
                    );
                    spawn_player_info_item(parent, &font.0, level_icon, "Level", PlayerLevelText);
                    spawn_player_info_item(parent, &font.0, xp_icon, "XP", PlayerXpText);
                    spawn_player_info_item(
                        parent,
                        &font.0,
                        defense_icon,
                        "Defense",
                        PlayerDefenseText,
                    );
                });
        })
        .insert(UiRoot);
}

fn spawn_slots_grid(
    parent: &mut ChildBuilder,
    font: &Handle<Font>,
    label: &str,
    count: usize,
    index: usize,
) {
    parent
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Px(80.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },))
        .with_children(|container| {
            container.spawn((
                Text::new(label),
                TextFont {
                    font: font.clone(),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            container
                .spawn((Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(70.0),
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    ..default()
                },))
                .with_children(|grid| {
                    for i in 0..count {
                        if i == 0 && index == 0 {
                            grid.spawn((
                                Node {
                                    width: Val::Px(50.0),
                                    height: Val::Px(50.0),
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                BorderColor(Color::BLACK),
                                FocusedItem,
                                GridSlot {
                                    x: i,
                                    y: index,
                                    item: None,
                                },
                                Name::new("GridItem"),
                                BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.5)),
                            ))
                            .with_children(|slot| {
                                slot.spawn((
                                    ImageNode {
                                        image: Handle::default(),
                                        ..default()
                                    },
                                    GridSlot {
                                        x: i,
                                        y: index,
                                        item: None,
                                    },
                                ));
                            });
                        } else {
                            grid.spawn((
                                Node {
                                    width: Val::Px(50.0),
                                    height: Val::Px(50.0),
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                Name::new("GridItem"),
                                BorderColor(Color::BLACK),
                                GridSlot {
                                    x: i,
                                    y: index,
                                    item: None,
                                },
                                BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.5)),
                            ))
                            .with_children(|slot| {
                                slot.spawn((
                                    ImageNode {
                                        image: Handle::default(),
                                        ..default()
                                    },
                                    GridSlot {
                                        x: i,
                                        y: index,
                                        item: None,
                                    },
                                ));
                            });
                        }
                    }
                });
        });
}

fn update_ui(
    level: Res<Level>,
    player_query: Query<(&Health, &Defense), With<Player>>,
    mut param_set: ParamSet<(
        Query<&mut Text, With<PlayerHealthText>>,
        Query<&mut Text, With<PlayerLevelText>>,
        Query<&mut Text, With<PlayerXpText>>,
        Query<&mut Text, With<PlayerDefenseText>>,
    )>,
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
}

fn navigate_loot_items(
    action_state: Res<ActionState<Action>>,
    mut commands: Commands,
    mut focused_item_query: Query<
        (&GridSlot, Entity, &mut Node, &mut BorderColor, &Children),
        With<FocusedItem>,
    >,
    mut grid_query: Query<(&GridSlot, Entity), (Without<FocusedItem>, Without<ImageNode>)>,
) {
    if focused_item_query.iter().next().is_none() {
        return;
    }
    let mut focused_item = focused_item_query.single_mut();
    let mut pressed = false;
    let new_focus = if action_state.just_pressed(&Action::NavigateUp) {
        pressed = true;
        Some((0, -1))
    } else if action_state.just_pressed(&Action::NavigateDown) {
        pressed = true;
        Some((0, 1))
    } else if action_state.just_pressed(&Action::NavigationLeft) {
        pressed = true;
        Some((-1, 0))
    } else if action_state.just_pressed(&Action::NavigationRight) {
        pressed = true;
        Some((1, 0))
    } else {
        None
    };
    if !pressed {
        return;
    }
    for (grid_slot, entity) in grid_query.iter_mut() {
        if let Some((dx, dy)) = new_focus {
            if (grid_slot.x as isize - dx) == focused_item.0.x as isize
                && (grid_slot.y as isize - dy) == focused_item.0.y as isize
            {
                commands.entity(entity).insert(FocusedItem);
                commands.entity(focused_item.1).remove::<FocusedItem>();
                if let Some(text_box_child) = focused_item.4.get(1) {
                    commands
                        .entity(focused_item.1)
                        .remove_children(&[*text_box_child]);

                    commands.entity(*text_box_child).despawn();
                }
                focused_item.2.border = UiRect::all(Val::Px(2.0));
                focused_item.3 .0 = Color::BLACK;
                break;
            }
        }
    }
}

fn highlight_focused_item(
    mut grid_query: Query<
        (&mut Node, &mut BorderColor, &FocusedItem, Entity, &GridSlot),
        Added<FocusedItem>,
    >,
    description_query: Query<&Description>,
    font: Res<UiFont>,
    mut commands: Commands,
) {
    for (mut node, mut border_color, _, entity, grid_slot) in grid_query.iter_mut() {
        node.border = UiRect::all(Val::Px(4.0));
        *border_color = BorderColor(Color::linear_rgb(1.0, 1.0, 0.0));
        if let Some(item_entity) = &grid_slot.item {
            if let Ok(description) = description_query.get(*item_entity) {
                commands
                    .entity(entity)
                    .with_child((
                        Node {
                            min_width: Val::Px(240.0),
                            height: Val::Px(120.0),
                            bottom: Val::Px(4.0),
                            left: Val::Px(6.0),
                            ..default()
                        },
                        Text::new(format!(
                            "{}\n {}",
                            description.name, description.description
                        )),
                        TextFont {
                            font: font.0.clone(),
                            font_size: 25.0,
                            ..default()
                        },
                        TextLayout {
                            justify: JustifyText::Center,
                            ..default()
                        },
                        GlobalZIndex(4),
                        TextColor(Color::BLACK),
                        BackgroundColor(Color::linear_rgba(1.0, 1.0, 1.0, 0.8)),
                    ))
                    .insert(DescriptionTextBox);
            }
        }
    }
}

fn set_up_loot_image(
    mut grid_query: Query<(&mut ImageNode, &mut GridSlot, &Parent)>,
    inventory_query: Query<&PlayerInventory, With<Player>>,
    sprite_query: Query<&Sprite>,
    mut grid_slot_query: Query<&mut GridSlot, Without<ImageNode>>,
) {
    if let Ok(player_inventory) = inventory_query.get_single() {
        for (mut image_node, grid_slot, parent) in grid_query.iter_mut() {
            let item_entity = match grid_slot.y {
                0 => player_inventory.health_potions.get(grid_slot.x),
                1 => player_inventory.speed_potions.get(grid_slot.x),
                2 => player_inventory.guns.get(grid_slot.x),
                3 => player_inventory.armors.get(grid_slot.x),
                _ => None,
            };

            if let Some(item_entity) = item_entity {
                if let Ok(sprite) = sprite_query.get(*item_entity) {
                    image_node.image = sprite.image.clone();
                    image_node.texture_atlas = sprite.texture_atlas.clone();
                    if let Ok(mut grid_slot) = grid_slot_query.get_mut(**parent) {
                        grid_slot.item = Some(*item_entity);
                    }
                } else {
                    println!(
                        "Failed to fetch sprite for item at slot ({}, {}).",
                        grid_slot.x, grid_slot.y
                    );
                }
            } else {
                image_node.image = Default::default();
                image_node.texture_atlas = None;
            }
        }
    } else {
        println!("Player inventory not found!");
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

fn setup_main_menu(mut commands: Commands, font: Res<UiFont>) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column, // Stack items vertically
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
                            font: font.0.clone(),
                            font_size: 50.0,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                    ));
                });
            parent.spawn((
                Text::new("Press Enter to Start"),
                TextFont {
                    font: font.0.clone(),
                    font_size: 50.0,
                    ..default()
                },
                MainMenuText,
                TextColor(Color::BLACK),
            ));
        })
        .insert(MainMenuItem);
}

fn handle_main_menu_buttons(
    action_state: Res<ActionState<Action>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if action_state.just_pressed(&Action::Confirm) {
        next_state.set(GameState::Initializing);
    }
}

fn blink_main_menu_text(
    mut text_query: Query<&mut TextColor, With<MainMenuText>>,
    time: Res<Time>,
) {
    if let Ok(mut text_color) = text_query.get_single_mut() {
        let flash_rate = 2.0;
        let alpha = (time.elapsed_secs() * flash_rate).sin().abs();
        text_color.0.set_alpha(alpha);
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
