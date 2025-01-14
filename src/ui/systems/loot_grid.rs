use crate::{
    configs::MAX_DEFENSE,
    game_state::GameState,
    input::Action,
    loot::{Description, LootType},
    player::{DamageBoost, Defense, Gold, Health, Player, PlayerInventory},
    resources::{GlobalTextureAtlas, Level, UiFont},
    ui::components::{
        DescriptionTextBox, FocusedItem, GridSlot, LootSaleEvent, PauseMenuRoot,
        PlayerDamageBoostText, PlayerDefenseText, PlayerGoldText, PlayerHealthText,
        PlayerLevelText, PlayerXpText, UiRoot,
    },
    utils::InGameEntity,
};
use bevy::{
    asset::Handle,
    color::Color,
    core::Name,
    hierarchy::{BuildChildren, ChildBuild, ChildBuilder, Children, Parent},
    image::Image,
    prelude::*,
};
use leafwing_input_manager::action_state::ActionState;

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
                    spawn_slots_grid(parent, &font.0, "Health Potions", 4, 0);
                    spawn_slots_grid(parent, &font.0, "Speed Potions", 4, 1);
                    spawn_slots_grid(parent, &font.0, "Guns", 4, 2);
                    spawn_slots_grid(parent, &font.0, "Armors", 4, 3);
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

pub fn spawn_slots_grid(
    parent: &mut ChildBuilder,
    font: &Handle<Font>,
    label: &str,
    count: usize,
    index: usize,
) {
    parent
        .spawn((
            Name::new("GridRow"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(80.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
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
                        let is_focused = i == 0 && index == 0;
                        spawn_single_grid_item(grid, i, index, is_focused);
                    }
                });
        });
}

pub fn spawn_single_grid_item(parent: &mut ChildBuilder, x: usize, y: usize, is_focused: bool) {
    let mut grid_item = parent.spawn((
        Node {
            width: Val::Px(50.0),
            height: Val::Px(50.0),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        Name::new("GridItem"),
        BorderColor(Color::BLACK),
        BorderRadius::all(Val::Px(4.0)),
        GridSlot { x, y, item: None },
        BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.5)),
    ));

    if is_focused {
        grid_item.insert(FocusedItem);
    }

    grid_item.with_children(|slot| {
        slot.spawn((
            ImageNode {
                image: Handle::default(),
                ..default()
            },
            GridSlot { x, y, item: None },
        ));
    });
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

pub fn handle_sell_focused_item(
    mut commands: Commands,
    action_state: Res<ActionState<Action>>,
    focused_item_query: Query<(&GridSlot, Entity, &Parent), With<FocusedItem>>,
    mut loot_sale_event_writer: EventWriter<LootSaleEvent>,
) {
    if action_state.just_pressed(&Action::SellLoot) {
        if let Ok((focused_slot, focused_entity, parent)) = focused_item_query.get_single() {
            if let Some(item_entity) = focused_slot.item {
                let loot_type = match focused_slot.y {
                    0 => LootType::Potion,
                    1 => LootType::Potion,
                    2 => LootType::Gun,
                    3 => LootType::Armor,
                    _ => return,
                };
                loot_sale_event_writer.send(LootSaleEvent(item_entity, loot_type));

                commands.entity(focused_entity).despawn_recursive();
                let grid_item = commands
                    .spawn((
                        Node {
                            width: Val::Px(50.0),
                            height: Val::Px(50.0),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        Name::new("GridItem"),
                        BorderColor(Color::BLACK),
                        FocusedItem,
                        BorderRadius::all(Val::Px(4.0)),
                        GridSlot {
                            x: focused_slot.x,
                            y: focused_slot.y,
                            item: None,
                        },
                        BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.5)),
                    ))
                    .with_child((
                        ImageNode {
                            image: Handle::default(),
                            ..default()
                        },
                        GridSlot {
                            x: focused_slot.x,
                            y: focused_slot.y,
                            item: None,
                        },
                    ))
                    .id();
                commands
                    .entity(parent.get())
                    .insert_children(focused_slot.x, &[grid_item]);
            }
        }
    }
}

pub fn navigate_loot_items(
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

                    commands.entity(*text_box_child).despawn_recursive();
                }
                focused_item.2.border = UiRect::all(Val::Px(2.0));
                focused_item.3 .0 = Color::BLACK;
                break;
            }
        }
    }
}

pub fn highlight_focused_item(
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
                    .with_children(|parent| {
                        parent
                            .spawn((
                                Node {
                                    min_width: Val::Px(220.0),
                                    height: Val::Px(120.0),
                                    bottom: Val::Px(5.0),
                                    border: UiRect::all(Val::Px(1.0)),
                                    left: Val::Px(10.0),
                                    ..default()
                                },
                                BorderRadius::all(Val::Px(6.0)),
                                BorderColor(Color::BLACK),
                                GlobalZIndex(4),
                                BackgroundColor(Color::srgba_u8(251, 255, 148, 238)),
                            ))
                            .with_child((
                                Text::new(format!(
                                    "{}\n {}",
                                    description.name, description.description
                                )),
                                TextFont {
                                    font: font.0.clone(),
                                    font_size: 30.0,
                                    ..default()
                                },
                                TextLayout {
                                    justify: JustifyText::Center,
                                    ..default()
                                },
                                TextColor(Color::BLACK),
                            ));
                    })
                    .insert(DescriptionTextBox);
            }
        }
    }
}

pub fn set_up_loot_image(
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
