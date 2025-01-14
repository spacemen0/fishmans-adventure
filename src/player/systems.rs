use bevy::{prelude::*, time::Stopwatch};
use leafwing_input_manager::prelude::*;

use super::*;
use crate::{
    armor::{Armor, ArmorStats},
    configs::*,
    gun::Gun,
    input::Action,
    loot::{MovingToPlayer, ReadyForPickup},
    potion::PotionType,
    resources::UiFont,
    ui::systems::in_game_ui::spawn_floating_text,
    utils::*,
};

pub fn handle_player_damaged_events(
    mut commands: Commands,
    mut player_query: Query<
        (
            &mut Health,
            &PlayerState,
            &Defense,
            &mut PlayerInventory,
            &Transform,
            Entity,
        ),
        (With<Player>, Without<InvincibilityEffect>),
    >,
    mut armor_query: Query<(&mut ArmorStats, Entity), With<Armor>>,
    mut events: EventReader<PlayerDamagedEvent>,
    font: Res<UiFont>,
) {
    if player_query.is_empty() || events.is_empty() {
        return;
    }
    let (mut health, _player_state, player_defense, mut inventory, player_transform, entity) =
        player_query.single_mut();

    for event in events.read() {
        if health.0 > 0 {
            let mut total_defense = player_defense.0;

            if let Some(active_armor_entity) = inventory.armors.get(inventory.active_armor_index) {
                if let Ok((mut armor_stats, armor_entity)) =
                    armor_query.get_mut(*active_armor_entity)
                {
                    total_defense += armor_stats.defense;
                    let damage_after_defense =
                        (event.damage as f32 * calculate_defense_percentage(total_defense)) as u32;
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
                        commands.entity(entity).insert(InvincibilityEffect(
                            Stopwatch::new(),
                            PLAYER_INVINCIBLE_TIME,
                        ));
                        spawn_floating_text(
                            &mut commands,
                            &font.0,
                            player_transform.translation,
                            format!("-{}", damage_after_defense.to_owned()),
                            None,
                        );
                    }
                }
            } else {
                let damage_after_defense = safe_subtract(event.damage, player_defense.0);
                health.0 = safe_subtract(health.0, damage_after_defense);
                if damage_after_defense > 0 {
                    spawn_floating_text(
                        &mut commands,
                        &font.0,
                        player_transform.translation,
                        format!("-{}", damage_after_defense.to_owned()),
                        None,
                    );
                }
            }
        } else {
            println!("Dead");
        }
    }
}

pub fn handle_leveling_up(
    mut event_reader: EventReader<PlayerLevelingUpEvent>,
    mut player_query: Query<(&mut Health, &mut Defense, &Transform), With<Player>>,
    mut commands: Commands,
    font: Res<UiFont>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut health, mut defense, transform) = player_query.single_mut();

    for event in event_reader.read() {
        let level = event.new_level;
        health.0 += calculate_health_increase(level);
        defense.0 += calculate_defense_increase(level);
        spawn_floating_text(
            &mut commands,
            &font.0,
            transform.translation,
            format!("Level Up!"),
            Some(Color::srgb_u8(0, 128, 0)),
        );
    }
}

pub fn handle_player_death(
    commands: Commands,
    all_entities: Query<Entity, With<InGameEntity>>,
    player_query: Query<(Entity, &Health), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if player_query.is_empty() {
        return;
    }
    let player = player_query.single();
    if player.1 .0 == 0 {
        cleanup_entities(commands, all_entities);
        next_state.set(GameState::End);
    }
}

pub fn handle_invincibility_effect(
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

pub fn handle_sprite_reset(mut player_query: Query<(&OriginalColor, &mut Sprite), With<Player>>) {
    if player_query.is_empty() {
        return;
    }
    let (color, mut sprite) = player_query.single_mut();
    sprite.color = color.0;
}

pub fn handle_acceleration_effect(
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

pub fn handle_player_movement(
    mut player_query: Query<(&mut Transform, &mut PlayerState, &Speed, &mut Sprite), With<Player>>,
    action_state: Res<ActionState<Action>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut transform, mut player_state, speed, mut sprite) = player_query.single_mut();

    let axis_pair = action_state.clamped_axis_pair(&Action::Move);
    if axis_pair != Vec2::ZERO {
        let movement = axis_pair * speed.0 as f32;
        sprite.flip_x = movement.x < 0.0;
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;

        *player_state = PlayerState::Run;
    } else {
        *player_state = PlayerState::Idle;
    }
}

pub fn mark_loot_for_pickup(
    mut commands: Commands,
    loot_query: Query<(Entity, &Transform), (With<Pickable>, Without<MovingToPlayer>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let player_pos = player_transform.translation.xy();

    for (loot_entity, loot_transform) in loot_query.iter() {
        let loot_pos = loot_transform.translation.xy();
        if player_pos.distance(loot_pos) <= 400.0 {
            if let Some(mut entity_commands) = commands.get_entity(loot_entity) {
                entity_commands.insert(MovingToPlayer);
            }
        }
    }
}

pub fn move_loot_to_player(
    mut commands: Commands,
    time: Res<Time>,
    mut loot_query: Query<(Entity, &mut Transform), (With<MovingToPlayer>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let player_transform = player_query.single();
    let player_pos = player_transform.translation.xy();

    for (loot_entity, mut transform) in loot_query.iter_mut() {
        let current_pos = transform.translation.xy();
        let direction = (player_pos - current_pos).normalize_or_zero();
        let distance = player_pos.distance(current_pos);

        let movement = direction * 1000.0 * time.delta_secs();
        transform.translation += movement.extend(LAYER2);

        if distance <= 20.0 {
            commands.entity(loot_entity).insert(ReadyForPickup);
        }
    }
}

pub fn handle_loot_pickup(
    mut commands: Commands,
    mut player_query: Query<&mut PlayerInventory, With<Player>>,
    loot_query: Query<
        (Entity, Option<&PotionType>, Option<&Gun>, Option<&Armor>),
        With<ReadyForPickup>,
    >,
) {
    if player_query.is_empty() {
        return;
    }

    let mut inventory = player_query.single_mut();

    for (loot_entity, potion_type, gun, armor) in loot_query.iter() {
        match (potion_type, gun, armor) {
            (Some(PotionType::Speed), _, _) => {
                if !inventory.speed_potions.contains(&loot_entity)
                    && inventory.speed_potions.len() < 4
                {
                    inventory.speed_potions.push(loot_entity);
                }
            }
            (Some(PotionType::Health), _, _) => {
                if !inventory.health_potions.contains(&loot_entity)
                    && inventory.health_potions.len() < 4
                {
                    inventory.health_potions.push(loot_entity);
                }
            }
            (_, Some(_), _) => {
                if !inventory.guns.contains(&loot_entity) && inventory.guns.len() < 4 {
                    inventory.guns.push(loot_entity);
                }
            }
            (_, _, Some(_)) => {
                if !inventory.armors.contains(&loot_entity) && inventory.armors.len() < 4 {
                    inventory.armors.push(loot_entity);
                }
            }
            _ => (),
        }

        commands
            .entity(loot_entity)
            .insert(Visibility::Hidden)
            .remove::<Pickable>()
            .remove::<MovingToPlayer>()
            .remove::<ReadyForPickup>();
    }
}

pub fn update_player_invincibility_visual(
    mut player_query: Query<&mut Sprite, (With<Player>, With<InvincibilityEffect>)>,
    time: Res<Time>,
) {
    if let Ok(mut sprite) = player_query.get_single_mut() {
        let flash_rate = 2.0;
        sprite.color = sprite
            .color
            .with_alpha((time.elapsed_secs() * flash_rate).sin().abs());
    }
}