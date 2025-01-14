use crate::{
    game_state::GameState,
    input::Action,
    resources::UiFont,
    ui::components::{
        BlinkingText, ControlWidget, DeathScreenRoot, MainMenuButton, MainMenuButtonIndex,
        MainMenuRoot, PauseMenuButton, PauseMenuButtonIndex, PauseMenuRoot,
    },
    utils::{cleanup_entities, InGameEntity},
};
use bevy::{
    app::AppExit,
    asset::AssetServer,
    color::{Alpha, Color},
    hierarchy::{BuildChildren, ChildBuild, DespawnRecursiveExt},
    prelude::*,
};
use leafwing_input_manager::action_state::ActionState;

pub fn setup_main_menu(mut commands: Commands, font: Res<UiFont>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            MainMenuRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(600.0),
                        height: Val::Px(260.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Fishmans Adventure"),
                        TextFont {
                            font: font.0.clone(),
                            font_size: 80.0,
                            ..default()
                        },
                        TextLayout {
                            justify: JustifyText::Center,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                    ));
                });

            parent
                .spawn((
                    Node {
                        width: Val::Px(600.0),
                        height: Val::Px(400.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderRadius::all(Val::Px(6.0)),
                ))
                .with_children(|parent| {
                    spawn_main_menu_button(parent, "Start", MainMenuButton::Start, &font.0, 0);
                    spawn_main_menu_button(parent, "Control", MainMenuButton::Control, &font.0, 1);
                    spawn_main_menu_button(parent, "Exit", MainMenuButton::Exit, &font.0, 2);
                });
        });
}

fn spawn_main_menu_button(
    parent: &mut ChildBuilder,
    button_text: &str,
    menu_button_type: MainMenuButton,
    font: &Handle<Font>,
    index: u8,
) {
    parent
        .spawn((
            Node {
                width: Val::Px(400.0),
                height: Val::Px(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderRadius::all(Val::Px(6.0)),
            MainMenuButtonIndex(index),
            BackgroundColor(Color::srgba_u8(255, 246, 225, 230)),
            menu_button_type,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(button_text),
                TextFont {
                    font: font.clone(),
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));
        });
}

pub fn handle_main_menu_buttons(
    mut next_state: ResMut<NextState<GameState>>,
    action_state: Res<ActionState<Action>>,
    mut selected_button: Local<u8>,
    mut query: Query<(&MainMenuButton, &mut BackgroundColor, &MainMenuButtonIndex)>,
    mut commands: Commands,
    mut exit: EventWriter<AppExit>,
    control_query: Query<&ControlWidget>,
    font: Res<UiFont>,
    mut visibility_query: Query<&mut Visibility, With<MainMenuRoot>>,
) {
    if !control_query.is_empty() || visibility_query.is_empty() {
        return;
    }
    let button_count = 3;
    let mut execute = false;

    if action_state.just_pressed(&Action::NavigateUp) {
        *selected_button = (*selected_button + button_count - 1) % button_count;
    }

    if action_state.just_pressed(&Action::NavigateDown) {
        *selected_button = (*selected_button + 1) % button_count;
    }

    if action_state.just_pressed(&Action::Confirm) {
        execute = true;
    }

    for (button, mut color, index) in query.iter_mut() {
        if index.0 == *selected_button {
            *color = BackgroundColor(Color::srgba_u8(204, 195, 176, 230));
            if execute {
                match button {
                    MainMenuButton::Control => {
                        setup_control_widget(&mut commands, font.0.clone());
                        let mut visibility = visibility_query.get_single_mut().unwrap();
                        *visibility = Visibility::Hidden;
                    }
                    MainMenuButton::Exit => {
                        exit.send(AppExit::Success);
                    }
                    MainMenuButton::Start => {
                        next_state.set(GameState::Initializing);
                        *selected_button = 0;
                    }
                }
            }
        } else {
            *color = BackgroundColor(Color::srgba_u8(255, 246, 225, 230));
        }
    }
}

pub fn setup_control_widget(commands: &mut Commands, font: Handle<Font>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ControlWidget,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(600.0),
                        height: Val::Px(620.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderRadius::all(Val::Px(10.0)),
                    BackgroundColor(Color::srgba_u8(217, 234, 146, 255)),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                width: Val::Auto,
                                height: Val::Auto,
                                border: UiRect::bottom(Val::Px(4.0)),
                                ..default()
                            },
                            BorderColor(Color::BLACK),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Control Bindings"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 40.0,
                                    ..default()
                                },
                                TextColor(Color::BLACK),
                            ));
                        });
                    spawn_control_binding_text(parent, "Move: W/A/S/D", &font);
                    spawn_control_binding_text(parent, "Switch Gun: Q", &font);
                    spawn_control_binding_text(parent, "Pause MenuL P", &font);
                    spawn_control_binding_text(parent, "Confirm: Enter", &font);
                    spawn_control_binding_text(parent, "Navigate: Arrow Keys/WASD", &font);
                    spawn_control_binding_text(parent, "Use Health Potion: 1", &font);
                    spawn_control_binding_text(parent, "Use Speed Potion: 2", &font);
                    spawn_control_binding_text(parent, "Toggle Loot Board: Tab", &font);
                    spawn_control_binding_text(parent, "Sell Loot: Del", &font);
                    parent
                        .spawn((
                            Node {
                                width: Val::Px(100.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BorderRadius::all(Val::Px(6.0)),
                            BackgroundColor(Color::srgba_u8(255, 246, 225, 230)),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Confirm"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 30.0,
                                    ..default()
                                },
                                TextColor(Color::BLACK),
                            ));
                        });
                });
        });
}

fn spawn_control_binding_text(parent: &mut ChildBuilder, text: &str, font: &Handle<Font>) {
    parent.spawn((
        Text::new(text),
        TextFont {
            font: font.clone(),
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::BLACK),
    ));
}

pub fn handle_control_widget(
    mut commands: Commands,
    action_state: Res<ActionState<Action>>,
    query: Query<Entity, With<ControlWidget>>,
    mut visibility_query: Query<&mut Visibility, With<MainMenuRoot>>,
) {
    if action_state.just_pressed(&Action::Confirm) {
        if let Ok(entity) = query.get_single() {
            if let Ok(mut visibility) = visibility_query.get_single_mut() {
                commands.entity(entity).despawn_recursive();
                *visibility = Visibility::Visible;
            }
        }
    }
}

pub fn blink_text(mut text_query: Query<&mut TextColor, With<BlinkingText>>, time: Res<Time>) {
    if let Ok(mut text_color) = text_query.get_single_mut() {
        let flash_rate = 2.0;
        let alpha = (time.elapsed_secs() * flash_rate).sin().abs();
        text_color.0.set_alpha(alpha);
    }
}

pub fn despawn_main_menu(
    mut commands: Commands,
    menu_items_query: Query<Entity, With<MainMenuRoot>>,
) {
    for e in menu_items_query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn handle_pause_input(
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

pub fn setup_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>, font: Res<UiFont>) {
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
                        height: Val::Px(300.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Name::new("PauseMenu"),
                    BorderRadius::all(Val::Px(10.0)),
                    BackgroundColor(Color::srgba_u8(237, 217, 165, 230)),
                ))
                .with_children(|parent| {
                    spawn_pause_menu_button(parent, "Resume", PauseMenuButton::Resume, &font.0, 0);
                    spawn_pause_menu_button(
                        parent,
                        "Restart",
                        PauseMenuButton::Restart,
                        &font.0,
                        1,
                    );
                    spawn_pause_menu_button(parent, "Quit", PauseMenuButton::Quit, &font.0, 2);
                });
        });
}

fn spawn_pause_menu_button(
    parent: &mut ChildBuilder,
    button_text: &str,
    menu_button_type: PauseMenuButton,
    font: &Handle<Font>,
    index: u8,
) {
    parent
        .spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderRadius::all(Val::Px(6.0)),
            PauseMenuButtonIndex(index),
            BackgroundColor(Color::srgba_u8(255, 246, 225, 230)),
            menu_button_type,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(button_text),
                TextFont {
                    font: font.clone(),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));
        });
}
pub fn pause_menu_navigation(
    mut next_state: ResMut<NextState<GameState>>,
    action_state: Res<ActionState<Action>>,
    all_entities: Query<Entity, With<InGameEntity>>,
    mut visibility_query: Query<&mut Visibility, With<PauseMenuRoot>>,
    mut selected_button: Local<u8>,
    mut query: Query<(
        &PauseMenuButton,
        &mut BackgroundColor,
        &PauseMenuButtonIndex,
    )>,
    commands: Commands,
    mut exit: EventWriter<AppExit>,
) {
    let button_count = query.iter().count() as u8;
    let mut execute = false;

    if action_state.just_pressed(&Action::NavigateUp) {
        *selected_button = (*selected_button + button_count - 1) % button_count;
    }

    if action_state.just_pressed(&Action::NavigateDown) {
        *selected_button = (*selected_button + 1) % button_count;
    }

    if action_state.just_pressed(&Action::Confirm) {
        execute = true;
    }
    for (button, mut color, index) in query.iter_mut() {
        if index.0 == *selected_button {
            *color = BackgroundColor(Color::srgba_u8(204, 195, 176, 230));
            if execute {
                let mut visibility = visibility_query.single_mut();
                match button {
                    PauseMenuButton::Resume => {
                        next_state.set(GameState::Combat);
                        *visibility = Visibility::Hidden;
                    }
                    PauseMenuButton::Restart => {
                        next_state.set(GameState::Initializing);
                        cleanup_entities(commands, all_entities);
                        return;
                    }
                    PauseMenuButton::Quit => {
                        exit.send(AppExit::Success);
                        return;
                    }
                }
                *selected_button = 0;
            }
        } else {
            *color = BackgroundColor(Color::srgba_u8(255, 246, 225, 230));
        }
    }
}

pub fn set_up_death_screen(mut commands: Commands, font: Res<UiFont>) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(800.0),
                        height: Val::Px(400.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    DeathScreenRoot,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Game Over"),
                        TextFont {
                            font: font.0.clone(),
                            font_size: 100.0,
                            ..default()
                        },
                        TextColor(Color::linear_rgb(1.0, 0.0, 0.0)),
                    ));
                    parent.spawn((
                        Text::new("Press Enter to Restart"),
                        TextFont {
                            font: font.0.clone(),
                            font_size: 60.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        BlinkingText,
                    ));
                });
        });
}

pub fn handle_death_screen_input(
    action_state: Res<ActionState<Action>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut query: Query<Entity, With<DeathScreenRoot>>,
    mut commands: Commands,
) {
    if action_state.just_pressed(&Action::Confirm) {
        let entity = query.single_mut();
        commands.entity(entity).despawn_recursive();
        next_state.set(GameState::MainMenu);
    }
}
