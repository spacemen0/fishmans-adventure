use std::time::Duration;

use bevy::{math::vec3, prelude::*, time::Stopwatch};
use leafwing_input_manager::prelude::*;

use crate::{
    armor::{Armor, ArmorStats},
    configs::{LAYER1, WH, WW},
    enemy::Collider,
    gun::{Gun, HasLifespan},
    input::Action,
    potion::Potion,
    resources::UiFont,
    state::GameState,
    utils::{
        calculate_defense_increase, calculate_health_increase, safe_subtract, InGameEntity,
        Pickable,
    },
};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Health(pub u32);
#[derive(Component)]
pub struct Speed(pub u32);
#[derive(Component)]
pub struct Defense(pub u32);
#[derive(Component)]
pub struct PlayerInventory {
    pub guns: Vec<Entity>,
    pub health_potions: Vec<Entity>,
    pub speed_potions: Vec<Entity>,
    pub armors: Vec<Entity>,
    pub active_gun_index: usize,
    pub active_armor_index: usize,
}

#[derive(Component)]
pub struct InvincibilityEffect(pub Stopwatch, pub f32);
#[derive(Component)]
pub struct AccelerationEffect(pub Stopwatch, pub f32, pub u32);

#[derive(Component, Default, Debug)]
pub enum PlayerState {
    #[default]
    Idle,
    Run,
}

#[derive(Event)]
pub struct PlayerDamagedEvent {
    pub damage: u32,
}
#[derive(Event)]
pub struct PlayerLevelingUpEvent {
    pub new_level: u32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDamagedEvent>()
            .add_event::<PlayerLevelingUpEvent>()
            .add_systems(
                Update,
                (
                    handle_player_death,
                    handle_player_input,
                    handle_player_damaged_events,
                    handle_invincibility_effect,
                    handle_acceleration_effect,
                    handle_leveling_up,
                    handle_loot_picking,
                )
                    .run_if(in_state(GameState::Combat).or_else(in_state(GameState::Town))),
            );
    }
}

fn handle_player_damaged_events(
    mut commands: Commands,
    mut player_query: Query<
        (
            &mut Health,
            &PlayerState,
            &Defense,
            &mut PlayerInventory,
            &Transform,
        ),
        With<Player>,
    >,
    mut armor_query: Query<(&mut ArmorStats, Entity), With<Armor>>,
    mut events: EventReader<PlayerDamagedEvent>,
    font: Res<UiFont>,
) {
    if player_query.is_empty() {
        return;
    }
    let (mut health, _player_state, player_defense, mut inventory, player_transform) =
        player_query.single_mut();

    for event in events.read() {
        if health.0 > 0 {
            let mut total_defense = player_defense.0;

            if let Some(active_armor_entity) = inventory.armors.get(inventory.active_armor_index) {
                if let Ok((mut armor_stats, armor_entity)) =
                    armor_query.get_mut(*active_armor_entity)
                {
                    total_defense += armor_stats.defense;

                    let damage_after_defense = safe_subtract(event.damage, total_defense);
                    health.0 = safe_subtract(health.0, damage_after_defense);

                    armor_stats.durability =
                        safe_subtract(armor_stats.durability, damage_after_defense);

                    if armor_stats.durability == 0 {
                        commands.entity(armor_entity).despawn();
                        let armor_to_remove = inventory.active_armor_index;
                        inventory.armors.remove(armor_to_remove);
                        if inventory.active_armor_index >= inventory.armors.len() {
                            inventory.active_armor_index = 0;
                        }
                    }
                    if damage_after_defense > 0 {
                        spawn_damage_text(
                            &mut commands,
                            &font.0,
                            player_transform.translation,
                            damage_after_defense,
                        );
                    }
                }
            } else {
                let damage_after_defense = safe_subtract(event.damage, player_defense.0);
                health.0 = safe_subtract(health.0, damage_after_defense);
                if damage_after_defense > 0 {
                    spawn_damage_text(
                        &mut commands,
                        &font.0,
                        player_transform.translation,
                        damage_after_defense,
                    );
                }
            }
        }
    }
}

fn handle_leveling_up(
    mut event_reader: EventReader<PlayerLevelingUpEvent>,
    mut player_query: Query<(&mut Health, &mut Defense), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut health, mut defense) = player_query.single_mut();

    for event in event_reader.read() {
        let level = event.new_level;
        health.0 += calculate_health_increase(level);
        defense.0 += calculate_defense_increase(level);
    }
}

fn spawn_damage_text(commands: &mut Commands, font: &Handle<Font>, position: Vec3, damage: u32) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("-{}", damage),
                TextStyle {
                    font: font.clone(),
                    font_size: 50.0,
                    color: Color::srgb(1.0, 0.0, 0.0),
                },
            ),
            transform: Transform {
                translation: position + Vec3::new(0.0, 50.0, 0.0),
                ..default()
            },
            ..default()
        },
        Collider { radius: 5 },
        HasLifespan::new(Duration::from_secs(1)),
        InGameEntity,
    ));
}

fn handle_player_death(
    player_query: Query<(&Health, Entity), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if player_query.is_empty() {
        return;
    }
    let player = player_query.single();
    if player.0 .0 == 0 {
        next_state.set(GameState::MainMenu);
    }
}

fn handle_invincibility_effect(
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(&mut InvincibilityEffect, Entity), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }
    let (mut invincibility_effect, entity) = player_query.single_mut();

    if invincibility_effect.0.elapsed_secs() >= invincibility_effect.1 {
        commands.entity(entity).remove::<InvincibilityEffect>();
    }
    invincibility_effect.0.tick(time.delta());
}

fn handle_acceleration_effect(
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(&mut AccelerationEffect, &mut Speed, Entity), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }
    let (mut acceleration_effect, mut speed, entity) = player_query.single_mut();

    if acceleration_effect.0.elapsed_secs() >= acceleration_effect.1 {
        commands.entity(entity).remove::<AccelerationEffect>();
        speed.0 -= acceleration_effect.2;
    }
    acceleration_effect.0.tick(time.delta());
}

pub fn handle_player_input(
    mut player_query: Query<(&mut Transform, &mut PlayerState, &Speed), With<Player>>,
    action_state: Res<ActionState<Action>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut transform, mut player_state, speed) = player_query.single_mut();

    let axis_pair = action_state.clamped_axis_pair(&Action::Move);
    if axis_pair != Vec2::ZERO {
        let desired_position = transform.translation.xy() + axis_pair * speed.0 as f32;
        let clamped_x = desired_position.x.clamp(-WW, WW);
        let clamped_y = desired_position.y.clamp(-WH, WH);
        transform.translation = vec3(clamped_x, clamped_y, transform.translation.z);

        transform.translation.z = LAYER1;
        *player_state = PlayerState::Run;
    } else {
        *player_state = PlayerState::Idle;
    }
}

fn handle_loot_picking(
    mut commands: Commands,
    loot_query: Query<
        (
            Entity,
            &Transform,
            Option<&Potion>,
            Option<&Gun>,
            Option<&Armor>,
        ),
        With<Pickable>,
    >,
    mut player_query: Query<(&mut PlayerInventory, &Transform), With<Player>>,
    action_state: Res<ActionState<Action>>,
) {
    if player_query.is_empty() {
        return;
    }

    // Check if the player pressed the "PickLoot" action
    if action_state.just_pressed(&Action::Interact) {
        let (mut inventory, player_transform) = player_query.single_mut();
        let player_pos = player_transform.translation.xy();

        // Iterate over nearby loot items
        for (loot_entity, loot_transform, potion, gun, armor) in loot_query.iter() {
            let loot_pos = loot_transform.translation.xy();

            // Check if loot is within a certain range of the player
            if player_pos.distance(loot_pos) <= 15.0 {
                if let Some(_potion) = potion {
                    inventory.speed_potions.push(loot_entity);
                }
                if let Some(_gun) = gun {
                    inventory.guns.push(loot_entity);
                }
                if let Some(_armor) = armor {
                    inventory.armors.push(loot_entity);
                }

                commands.entity(loot_entity).insert(Visibility::Hidden);
                commands.entity(loot_entity).remove::<Pickable>();
            }
        }
    }
}
