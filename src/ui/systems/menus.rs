use crate::{
    game_state::GameState,
    input::Action,
    resources::UiFont,
    ui::components::{
        BlinkingText, DeathScreenRoot, MainMenuRoot, MenuButton, PauseMenuButtonIndex,
        PauseMenuRoot,
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
            parent.spawn((
                Text::new("Press Enter to Start"),
                TextFont {
                    font: font.0.clone(),
                    font_size: 50.0,
                    ..default()
                },
                Node {
                    top: Val::Px(50.0),
                    ..default()
                },
                BlinkingText,
                TextColor(Color::BLACK),
            ));
        });
}

pub fn handle_main_menu_buttons(
    action_state: Res<ActionState<Action>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if action_state.just_pressed(&Action::Confirm) {
        next_state.set(GameState::Initializing);
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
                    spawn_pause_menu_button(parent, "Resume", MenuButton::Resume, &font.0, 0);
                    spawn_pause_menu_button(parent, "Restart", MenuButton::Restart, &font.0, 1);
                    spawn_pause_menu_button(parent, "Quit", MenuButton::Quit, &font.0, 2);
                });
        });
}

fn spawn_pause_menu_button(
    parent: &mut ChildBuilder,
    button_text: &str,
    menu_button_type: MenuButton,
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
    mut query: Query<(&MenuButton, &mut BackgroundColor, &PauseMenuButtonIndex)>,
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
    for (_, (button, mut color, index)) in query.iter_mut().enumerate() {
        if index.0 == *selected_button {
            *color = BackgroundColor(Color::srgba_u8(204, 195, 176, 230));
            if execute {
                let mut visibility = visibility_query.single_mut();
                match button {
                    MenuButton::Resume => {
                        next_state.set(GameState::Combat);
                        *visibility = Visibility::Hidden;
                    }
                    MenuButton::Restart => {
                        next_state.set(GameState::Initializing);
                        cleanup_entities(commands, all_entities);
                        return;
                    }
                    MenuButton::Quit => {
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
