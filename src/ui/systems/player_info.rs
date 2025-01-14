use bevy::prelude::{default, AlignItems, BackgroundColor, BorderColor, Commands, Component, FlexDirection, Font, GlobalZIndex, ImageNode, NextState, Node, ParamSet, Query, Res, ResMut, Text, TextColor, TextFont, TextureAtlas, UiRect, Val, Visibility, With};
use leafwing_input_manager::action_state::ActionState;
use bevy::core::Name;
use bevy::color::Color;
use bevy::hierarchy::{BuildChildren, ChildBuild, ChildBuilder};
use bevy::asset::Handle;
use bevy::image::Image;
use crate::configs::MAX_DEFENSE;
use crate::game_state::GameState;
use crate::input::Action;
use crate::player::{DamageBoost, Defense, Gold, Health, Player};
use crate::resources::{GlobalTextureAtlas, Level, UiFont};
use crate::ui::components::{PauseMenuRoot, PlayerDamageBoostText, PlayerDefenseText, PlayerGoldText, PlayerHealthText, PlayerLevelText, PlayerXpText, UiRoot};
use crate::ui::systems::loot_grid;
use crate::utils::InGameEntity;

pub fn setup_ui(mut commands: Commands, font: Res<UiFont>, handle: Res<GlobalTextureAtlas>) {
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
            BackgroundColor(Color::srgba_u8(237, 217, 165, 240)),
            InGameEntity,
        ))
        .with_children(|parent| {
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
                    loot_grid::spawn_slots_grid(parent, &font.0, "Health Potions", 4, 0);
                    loot_grid::spawn_slots_grid(parent, &font.0, "Speed Potions", 4, 1);
                    loot_grid::spawn_slots_grid(parent, &font.0, "Guns", 4, 2);
                    loot_grid::spawn_slots_grid(parent, &font.0, "Armors", 4, 3);
                });

            parent
                .spawn((
                    Node {
                        width: Val::Percent(70.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
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
                        handle.image.clone().unwrap(),
                        "Gold",
                        PlayerGoldText,
                        TextureAtlas {
                            layout: handle.layout_16x16.clone().unwrap(),
                            index: 161,
                        },
                        "Gold is the only currency in the game. "
                    );
                    spawn_player_info_item(
                        parent,
                        &font.0,
                        handle.image.clone().unwrap(),
                        "Health",
                        PlayerHealthText,
                        TextureAtlas {
                            layout: handle.layout_16x16.clone().unwrap(),
                            index: 162,
                        },
                        "Health is the amount of damage you can take"
                    );
                    spawn_player_info_item(
                        parent,
                        &font.0,
                        handle.image.clone().unwrap(),
                        "Level",
                        PlayerLevelText,
                        TextureAtlas {
                            layout: handle.layout_16x16.clone().unwrap(),
                            index: 165,
                        },
                        "Level is increased by gaining XP. You gain Damage Boost or Defense as you level up.",
                    );
                    spawn_player_info_item(
                        parent,
                        &font.0,
                        handle.image.clone().unwrap(),
                        "XP",
                        PlayerXpText,
                        TextureAtlas {
                            layout: handle.layout_16x16.clone().unwrap(),
                            index: 164,
                        },
                        "XP is gained by killing enemies. You level up when you reach the required XP threshold.",
                    );
                    spawn_player_info_item(
                        parent,
                        &font.0,
                        handle.image.clone().unwrap(),
                        "Defense",
                        PlayerDefenseText,
                        TextureAtlas {
                            layout: handle.layout_16x16.clone().unwrap(),
                            index: 163,
                        },
                        format!("Defense reduces the amount of damage taken from enemies by percentage. You get 80% damage reduction at maximum defense which is {}.",MAX_DEFENSE).as_str(),
                    );
                    spawn_player_info_item(
                        parent,
                        &font.0,
                        handle.image.clone().unwrap(),
                        "DamageBoost",
                        PlayerDamageBoostText,
                        TextureAtlas {
                            layout: handle.layout_16x16.clone().unwrap(),
                            index: 163,
                        },
                        "DamageBoost increases the amount of damage dealt to enemies by percentage. This is simply added to your damage.",
                    );
                });
        })
        .insert(UiRoot);
}

pub fn update_ui(
    level: Res<Level>,
    player_query: Query<(&Health, &Defense, &Gold, &DamageBoost), With<Player>>,
    mut param_set: ParamSet<(
        Query<&mut Text, With<PlayerHealthText>>,
        Query<&mut Text, With<PlayerLevelText>>,
        Query<&mut Text, With<PlayerXpText>>,
        Query<&mut Text, With<PlayerDefenseText>>,
        Query<&mut Text, With<PlayerGoldText>>,
        Query<&mut Text, With<PlayerDamageBoostText>>,
    )>,
    mut pause_menu_query: Query<&mut Visibility, With<PauseMenuRoot>>,
) {
    if let Ok((health, defense, gold, damage_boost)) = player_query.get_single() {
        if let Ok(mut health_text) = param_set.p0().get_single_mut() {
            *health_text = format!("Health: {}", health.0).into();
        }
        if let Ok(mut defense_text) = param_set.p3().get_single_mut() {
            *defense_text = format!("Defense: {}", defense.0).into();
        }
        if let Ok(mut gold_text) = param_set.p4().get_single_mut() {
            *gold_text = format!("Gold: {}", gold.0).into();
        }
        if let Ok(mut damage_boost_text) = param_set.p5().get_single_mut() {
            *damage_boost_text = format!("DamageBoost: {}", damage_boost.0).into();
        }
    }

    if let Ok(mut level_text) = param_set.p1().get_single_mut() {
        *level_text = format!("Level: {}", level.level()).into();
    }

    if let Ok(mut xp_text) = param_set.p2().get_single_mut() {
        *xp_text = format!("XP: {}/{}", level.current_xp(), level.xp_threshold()).into();
    }
    if let Ok(mut visibility) = pause_menu_query.get_single_mut() {
        if *visibility == Visibility::Visible {
            *visibility = Visibility::Hidden;
        }
    }
}

pub fn toggle_loot_ui_visibility(
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

pub fn spawn_player_info_item(
    parent: &mut ChildBuilder,
    font: &Handle<Font>,
    image: Handle<Image>,
    label: &str,

    component: impl Component,
    texture_atlas: TextureAtlas,
    description: &str,
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
                    image: image.clone(),
                    texture_atlas: Some(texture_atlas.clone()),
                    ..default()
                },
                Node {
                    width: Val::Px(32.0),
                    height: Val::Px(32.0),
                    top: Val::Px(4.0),
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

    // Add description text below the label
    parent.spawn((
        Text::new(description.to_string()),
        TextFont {
            font: font.clone(),
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.5)),
    ));
}