use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    armor::{Armor, ArmorStats},
    enemy::Enemy,
    input::Action,
    player::{Defense, Health, Player, PlayerInventory},
    resources::{Level, UiFont, Wave},
    state::GameState,
    utils::InGameEntity,
};

pub struct GuiPlugin;

#[derive(Component)]
struct DebugText;
#[derive(Component)]
struct PotionDisplay;
#[derive(Component)]
struct MainMenuItem;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
            .add_systems(
                Update,
                handle_main_menu_buttons.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(
                OnEnter(GameState::Initializing),
                (spawn_debug_text, setup_potion_display),
            )
            .add_systems(
                Update,
                (update_debug_text, update_potion_display).run_if(in_state(GameState::Combat)),
            )
            .add_systems(
                Update,
                (handle_pause_input, handle_game_restart)
                    .run_if(in_state(GameState::Combat).or_else(in_state(GameState::Paused))),
            );
    }
}

fn spawn_debug_text(mut commands: Commands, font: Res<UiFont>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            InGameEntity,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(186.0),
                        height: Val::Px(129.0),
                        align_items: AlignItems::Start,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Start,
                        padding: UiRect::all(Val::Px(2.0)),
                        margin: UiRect::px(5.0, 5.0, 5.0, 0.0),
                        ..default()
                    },
                    background_color: BackgroundColor::from(Color::BLACK.with_alpha(0.8)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Hello Bevy!",
                            TextStyle {
                                font: font.0.clone(),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        DebugText,
                    ));
                });
        });
}

fn update_debug_text(
    mut query: Query<&mut Text, With<DebugText>>,
    diagnostics: Res<DiagnosticsStore>,
    enemy_query: Query<(), With<Enemy>>,
    player_query: Query<(&Health, &Defense), With<Player>>,
    wave: Res<Wave>,
    level: Res<Level>,
) {
    if query.is_empty() || player_query.is_empty() || enemy_query.is_empty() {
        return;
    }
    let player_health = player_query.single().0 .0;
    let player_defense = player_query.single().1 .0;
    let current_wave = wave.number;
    let enemies_total = wave.enemies_total;
    let enemies_remaining = wave.enemies_left;
    let current_level = level.level();
    let mut text = query.single_mut();
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            text.sections[0].value =
                format!("Fps: {value:.2}\nWave: {current_wave}\nEnemies left: {enemies_remaining}/{enemies_total}\nHealth: {player_health}\nDefense: {player_defense}\nLevel: {current_level}");
        }
    }
}

fn setup_potion_display(mut commands: Commands, font: Res<UiFont>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(10.0),
                    top: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            InGameEntity,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Potions: ",
                    TextStyle {
                        font: font.0.clone(),
                        font_size: 15.0,
                        color: Color::WHITE,
                    },
                ),
                PotionDisplay,
            ));
        });
}

fn update_potion_display(
    mut query: Query<&mut Text, With<PotionDisplay>>,
    player_query: Query<&PlayerInventory, With<Player>>,
    armor_query: Query<&ArmorStats, With<Armor>>,
) {
    let mut text = query.single_mut();
    let player_inventory = player_query.single();
    let health_potions_count = player_inventory.health_potions.len();
    let speed_potions_count = player_inventory.speed_potions.len();

    let armor_info = if let Some(active_armor_entity) = player_inventory
        .armors
        .get(player_inventory.active_armor_index)
    {
        if let Ok(armor_stats) = armor_query.get(*active_armor_entity) {
            format!(
                "Armor Defense: {}, Durability: {}",
                armor_stats.defense, armor_stats.durability
            )
        } else {
            "No Armor".to_string()
        }
    } else {
        "No Armor".to_string()
    };

    text.sections[0].value = format!(
        "Health Potions: {}, Speed Potions: {}\n{}",
        health_potions_count, speed_potions_count, armor_info
    );
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
    action_state: Res<ActionState<Action>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if action_state.just_pressed(&Action::Restart) {
        next_state.set(GameState::Initializing);
    }
}
