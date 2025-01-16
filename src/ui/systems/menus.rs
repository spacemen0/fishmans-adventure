use std::time::Duration;

use crate::{
    armor::{Armor, ArmorStats},
    configs::{SPRITE_SCALE_FACTOR, UI_BG_COLOR},
    game_state::GameState,
    gun::{BulletStats, Gun, GunStats, GunType},
    input::Action,
    loot::Value,
    player::{Gold, PlayerInventory, PlayerLevelingUpEvent},
    potion::{Potion, PotionStats, PotionType},
    resources::{GameMode, GlobalTextureAtlas, Level, UiFont},
    ui::components::{
        BlinkingText, ControlWidget, DeathScreenRoot, FloatingTextBox, MainMenuButton,
        MainMenuButtonIndex, MainMenuRoot, PauseMenuButton, PauseMenuButtonIndex, PauseMenuRoot,
        ShopMenuButton, ShopMenuButtonIndex, ShopMenuRoot,
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
            BackgroundColor(Color::srgb_u8(UI_BG_COLOR.0, UI_BG_COLOR.1, UI_BG_COLOR.2)),
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
                    spawn_main_menu_button(
                        parent,
                        "Normal Mode",
                        MainMenuButton::StartNormal,
                        &font.0,
                        0,
                    );
                    spawn_main_menu_button(
                        parent,
                        "Forever Mode",
                        MainMenuButton::StartForever,
                        &font.0,
                        1,
                    );
                    spawn_main_menu_button(parent, "Control", MainMenuButton::Control, &font.0, 2);
                    spawn_main_menu_button(parent, "Exit", MainMenuButton::Exit, &font.0, 3);
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
                width: Val::Px(360.0),
                height: Val::Px(70.0),
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
    mut game_mode: ResMut<GameMode>,
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
    let button_count = 4;
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
                    MainMenuButton::StartNormal => {
                        *game_mode = GameMode::Normal;
                        next_state.set(GameState::Initializing);
                    }
                    MainMenuButton::StartForever => {
                        *game_mode = GameMode::Forever;
                        next_state.set(GameState::Initializing);
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
                    spawn_control_binding_text(parent, "Toggle Shop: O", &font);
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
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb_u8(UI_BG_COLOR.0, UI_BG_COLOR.1, UI_BG_COLOR.2)),
        ))
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

pub fn set_up_win_screen(mut commands: Commands, font: Res<UiFont>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb_u8(UI_BG_COLOR.0, UI_BG_COLOR.1, UI_BG_COLOR.2)),
        ))
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
                        Text::new("Congrats you don't suck ass"),
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

pub fn handle_shop_input(
    action_state: Res<ActionState<Action>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut query: Query<&mut Visibility, With<ShopMenuRoot>>,
    current_state: Res<State<GameState>>,
) {
    if action_state.just_pressed(&Action::ToggleShop) {
        let mut visibility = query.single_mut();
        if current_state.get() == &GameState::Combat {
            next_state.set(GameState::Shopping);

            *visibility = Visibility::Visible;
        } else {
            next_state.set(GameState::Combat);
            *visibility = Visibility::Hidden;
        }
    }
}

pub fn setup_shop_menu(mut commands: Commands, font: Res<UiFont>) {
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
            ShopMenuRoot,
            InGameEntity,
        ))
        .with_children(|parent| {
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
                    BorderRadius::all(Val::Px(10.0)),
                    BackgroundColor(Color::srgba_u8(237, 217, 165, 230)),
                ))
                .with_children(|parent| {
                    spawn_shop_menu_button(
                        parent,
                        "Health Potion - 50g",
                        ShopMenuButton::BuyHealthPotion,
                        &font.0,
                        0,
                    );
                    spawn_shop_menu_button(
                        parent,
                        "Speed Potion - 50g",
                        ShopMenuButton::BuySpeedPotion,
                        &font.0,
                        1,
                    );
                    spawn_shop_menu_button(
                        parent,
                        "Gun - 200g",
                        ShopMenuButton::BuyGun,
                        &font.0,
                        2,
                    );
                    spawn_shop_menu_button(
                        parent,
                        "Armor - 100g",
                        ShopMenuButton::BuyArmor,
                        &font.0,
                        3,
                    );
                    spawn_shop_menu_button(
                        parent,
                        "Buy XP - 400g",
                        ShopMenuButton::BuyArmor,
                        &font.0,
                        4,
                    );
                });
        });
}

fn spawn_shop_menu_button(
    parent: &mut ChildBuilder,
    button_text: &str,
    menu_button_type: ShopMenuButton,
    font: &Handle<Font>,
    index: u8,
) {
    parent
        .spawn((
            Node {
                width: Val::Px(400.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderRadius::all(Val::Px(6.0)),
            ShopMenuButtonIndex(index),
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

pub fn handle_shop_menu_buttons(
    mut commands: Commands,
    mut player_query: Query<(&mut PlayerInventory, &mut Gold)>,
    action_state: Res<ActionState<Action>>,
    mut selected_button: Local<u8>,
    mut query: Query<(&ShopMenuButton, &mut BackgroundColor, &ShopMenuButtonIndex)>,
    texture_atlases: Res<GlobalTextureAtlas>,
    mut menu_query: Query<&mut Visibility, With<ShopMenuRoot>>,
    font: Res<UiFont>,
    next_state: ResMut<NextState<GameState>>,
    level: ResMut<Level>,
    ew: EventWriter<PlayerLevelingUpEvent>,
) {
    let button_count = 5;
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

    let mut visibility = menu_query.get_single_mut().unwrap();

    for (button, mut color, index) in query.iter_mut() {
        if index.0 == *selected_button {
            *color = BackgroundColor(Color::srgba_u8(204, 195, 176, 230));
            if execute {
                if let Ok((mut inventory, mut gold)) = player_query.get_single_mut() {
                    match button {
                        ShopMenuButton::BuyHealthPotion => {
                            handle_buy_health_potion(
                                &mut commands,
                                &mut inventory,
                                &mut gold,
                                &texture_atlases,
                                &mut visibility,
                                &font,
                                next_state,
                            );
                            break;
                        }
                        ShopMenuButton::BuySpeedPotion => {
                            handle_buy_speed_potion(
                                &mut commands,
                                &mut inventory,
                                &mut gold,
                                &texture_atlases,
                                &mut visibility,
                                &font,
                                next_state,
                            );
                            break;
                        }
                        ShopMenuButton::BuyGun => {
                            handle_buy_gun(
                                &mut commands,
                                &mut inventory,
                                &mut gold,
                                &texture_atlases,
                                &mut visibility,
                                &font,
                                next_state,
                            );
                            break;
                        }
                        ShopMenuButton::BuyArmor => {
                            handle_buy_armor(
                                &mut commands,
                                &mut inventory,
                                &mut gold,
                                &texture_atlases,
                                &mut visibility,
                                &font,
                                next_state,
                            );
                            break;
                        }
                        ShopMenuButton::BuyXP => {
                            handle_buy_xp(
                                &mut commands,
                                level,
                                &mut gold,
                                &mut visibility,
                                &font,
                                next_state,
                                ew,
                            );
                            break;
                        }
                    }
                }
            }
        } else {
            *color = BackgroundColor(Color::srgba_u8(255, 246, 225, 230));
        }
    }
}

fn handle_buy_xp(
    commands: &mut Commands,
    mut level: ResMut<Level>,
    gold: &mut Gold,
    visibility: &mut Visibility,
    font: &UiFont,
    mut next_state: ResMut<NextState<GameState>>,
    mut ev_level_up: EventWriter<PlayerLevelingUpEvent>,
) {
    if gold.0 >= 400 {
        gold.0 -= 400;
        if level.add_xp(100) {
            ev_level_up.send(PlayerLevelingUpEvent {
                new_level: level.level(),
            });
        }
        *visibility = Visibility::Hidden;
        next_state.set(GameState::Combat);
        spawn_floating_text_box(commands, &font.0, "Item Bought!".to_owned());
    } else {
        spawn_floating_text_box(commands, &font.0, "Not Enough Gold".to_owned());
    }
}

fn handle_buy_health_potion(
    commands: &mut Commands,
    inventory: &mut PlayerInventory,
    gold: &mut Gold,
    texture_atlases: &GlobalTextureAtlas,
    visibility: &mut Visibility,
    font: &UiFont,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if gold.0 >= 50 {
        if inventory.health_potions.len() < 4 {
            gold.0 -= 50;
            let health_potion = commands
                .spawn((
                    Name::new("HealthPotion"),
                    Potion,
                    Value(5),
                    Sprite {
                        image: texture_atlases.image.clone().unwrap(),
                        texture_atlas: Some(TextureAtlas {
                            layout: texture_atlases.layout_16x16.clone().unwrap(),
                            index: 96,
                        }),
                        ..default()
                    },
                    Visibility::Hidden,
                    Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                    PotionStats {
                        effect_duration: 0.0,
                        effect_amount: 10,
                    },
                    PotionType::Health,
                ))
                .id();
            inventory.health_potions.push(health_potion);
            *visibility = Visibility::Hidden;
            next_state.set(GameState::Combat);
            spawn_floating_text_box(commands, &font.0, "Item Bought!".to_owned());
        } else {
            spawn_floating_text_box(commands, &font.0, "Inventory Full".to_owned());
        }
    } else {
        spawn_floating_text_box(commands, &font.0, "Not Enough Gold".to_owned());
    }
}

fn handle_buy_speed_potion(
    commands: &mut Commands,
    inventory: &mut PlayerInventory,
    gold: &mut Gold,
    texture_atlases: &GlobalTextureAtlas,
    visibility: &mut Visibility,
    font: &UiFont,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if gold.0 >= 50 {
        if inventory.speed_potions.len() < 4 {
            gold.0 -= 50;
            let speed_potion = commands
                .spawn((
                    Name::new("SpeedPotion"),
                    Potion,
                    Value(5),
                    Sprite {
                        image: texture_atlases.image.clone().unwrap(),
                        texture_atlas: Some(TextureAtlas {
                            layout: texture_atlases.layout_16x16.clone().unwrap(),
                            index: 97,
                        }),
                        ..default()
                    },
                    Visibility::Hidden,
                    Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                    PotionStats {
                        effect_duration: 1.0,
                        effect_amount: 10,
                    },
                    PotionType::Speed,
                ))
                .id();
            inventory.speed_potions.push(speed_potion);
            *visibility = Visibility::Hidden;
            next_state.set(GameState::Combat);
            spawn_floating_text_box(commands, &font.0, "Item Bought!".to_owned());
        } else {
            spawn_floating_text_box(commands, &font.0, "Inventory Full".to_owned());
        }
    } else {
        spawn_floating_text_box(commands, &font.0, "Not Enough Gold".to_owned());
    }
}

fn handle_buy_gun(
    commands: &mut Commands,
    inventory: &mut PlayerInventory,
    gold: &mut Gold,
    texture_atlases: &GlobalTextureAtlas,
    visibility: &mut Visibility,
    font: &UiFont,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if gold.0 >= 200 {
        if inventory.guns.len() < 4 {
            gold.0 -= 200;
            let gun = commands
                .spawn((
                    Name::new("RandomGun"),
                    Gun,
                    Value(10),
                    Sprite {
                        image: texture_atlases.image.clone().unwrap(),
                        texture_atlas: Some(TextureAtlas {
                            layout: texture_atlases.layout_16x16.clone().unwrap(),
                            index: 65,
                        }),
                        ..default()
                    },
                    Visibility::Hidden,
                    Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                    GunType::SingleDirectionSpread,
                    GunStats {
                        bullets_per_shot: 5,
                        firing_interval: 0.5,
                        bullet_spread: 0.2,
                    },
                    BulletStats {
                        speed: 20,
                        damage: 20,
                        lifespan: 0.6,
                    },
                ))
                .id();
            inventory.guns.push(gun);
            *visibility = Visibility::Hidden;
            next_state.set(GameState::Combat);
            spawn_floating_text_box(commands, &font.0, "Item Bought!".to_owned());
        } else {
            spawn_floating_text_box(commands, &font.0, "Inventory Full".to_owned());
        }
    } else {
        spawn_floating_text_box(commands, &font.0, "Not Enough Gold".to_owned());
    }
}

fn handle_buy_armor(
    commands: &mut Commands,
    inventory: &mut PlayerInventory,
    gold: &mut Gold,
    texture_atlases: &GlobalTextureAtlas,
    visibility: &mut Visibility,

    font: &UiFont,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if gold.0 >= 100 {
        if inventory.armors.len() < 4 {
            gold.0 -= 100;
            let armor = commands
                .spawn((
                    Name::new("RandomArmor"),
                    Armor,
                    Value(10),
                    ArmorStats {
                        defense: 2,
                        durability: 20,
                    },
                    Sprite {
                        image: texture_atlases.image.clone().unwrap(),
                        texture_atlas: Some(TextureAtlas {
                            layout: texture_atlases.layout_16x16.clone().unwrap(),
                            index: 98,
                        }),
                        ..default()
                    },
                    Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                    Visibility::Hidden,
                ))
                .id();
            inventory.armors.push(armor);
            *visibility = Visibility::Hidden;
            next_state.set(GameState::Combat);
            spawn_floating_text_box(commands, &font.0, "Item Bought!".to_owned());
        } else {
            spawn_floating_text_box(commands, &font.0, "Inventory Full".to_owned());
        }
    } else {
        spawn_floating_text_box(commands, &font.0, "Not Enough Gold".to_owned());
    }
}

fn spawn_floating_text_box(commands: &mut Commands, font: &Handle<Font>, message: String) {
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
            FloatingTextBox::new(Duration::from_secs(1)),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderRadius::all(Val::Px(10.0)),
                    BackgroundColor(Color::srgba_u8(0, 0, 0, 200)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(message),
                        TextFont {
                            font: font.clone(),
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

pub fn despawn_floating_text_box(
    mut commands: Commands,
    text_box_query: Query<(Entity, &FloatingTextBox)>,
) {
    for (entity, text_box) in text_box_query.iter() {
        if text_box.spawn_time.elapsed() > text_box.lifespan {
            commands.entity(entity).try_despawn_recursive();
        }
    }
}

pub fn despawn_all_floating_text_boxes(
    mut commands: Commands,
    text_box_query: Query<Entity, With<FloatingTextBox>>,
) {
    for entity in text_box_query.iter() {
        commands.entity(entity).try_despawn_recursive();
    }
}
