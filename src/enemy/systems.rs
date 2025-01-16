use super::{components::*, presets::*};
use crate::{
    audio::AudioEvent,
    configs::*,
    enemy::EnemyBuilder,
    game_state::GameState,
    gun::{BulletDirection, BulletStats, HasLifespan},
    loot::LootPool,
    player::{Health, InvincibilityEffect, Player, PlayerDamagedEvent, PlayerLevelingUpEvent},
    resources::{GameMode, GlobalTextureAtlas, Level, Wave},
    utils::{apply_movement, clamp_position, get_random_position_around, InGameEntity},
};
use bevy::prelude::*;
use rand::{random, Rng};
use std::time::Duration;

pub fn update_enemy_movement(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<
        (
            Entity,
            &Enemy,
            &mut Transform,
            &mut EnemyState,
            Option<&RangedBehavior>,
            Option<&TrailAbility>,
        ),
        (Without<Player>, Without<ChargeAbility>),
    >,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation;

        let enemy_positions: Vec<(Entity, Vec3)> = enemy_query
            .iter()
            .map(|(entity, _, transform, _, _, _)| (entity, transform.translation))
            .collect();

        for (entity, enemy, mut transform, mut state, ranged_behavior, trail_ability) in
            enemy_query.iter_mut()
        {
            let mut movement = Vec2::ZERO;

            match &mut *state {
                EnemyState::Wandering { direction, timer } => {
                    timer.tick(time.delta());

                    let distance_to_player = transform.translation.distance(player_pos);
                    if distance_to_player < 500.0 && trail_ability.is_none() {
                        *state = EnemyState::Pursuing;
                    } else {
                        if timer.just_finished() {
                            let near_left = transform.translation.x < -WW + REPEL_MARGIN;
                            let near_right = transform.translation.x > WW - REPEL_MARGIN;
                            let near_top = transform.translation.y > WH - REPEL_MARGIN;
                            let near_bottom = transform.translation.y < -WH + REPEL_MARGIN;

                            if near_left || near_right || near_top || near_bottom {
                                let mut new_direction = Vec2::ZERO;
                                if near_left {
                                    new_direction.x += 1.0;
                                }
                                if near_right {
                                    new_direction.x -= 1.0;
                                }
                                if near_top {
                                    new_direction.y -= 1.0;
                                }
                                if near_bottom {
                                    new_direction.y += 1.0;
                                }

                                let random_angle = rand::thread_rng().gen_range(-0.5..0.5);
                                let rotation = Mat2::from_angle(random_angle);
                                *direction = (rotation * new_direction.normalize()).normalize();
                            } else {
                                let angle_change = rand::thread_rng().gen_range(-0.8..0.8);
                                let rotation = Mat2::from_angle(angle_change);
                                *direction = (rotation * *direction).normalize();
                            }

                            timer.set_duration(Duration::from_secs_f32(
                                rand::thread_rng().gen_range(1.8..2.5),
                            ));
                            timer.reset();
                        }

                        movement = *direction * enemy.speed as f32 * 0.5;
                    }
                }
                EnemyState::Pursuing => {
                    let dir = (player_pos - transform.translation).normalize();
                    movement = dir.truncate() * enemy.speed as f32;
                }
                EnemyState::MaintainingDistance => {
                    if let Some(ranged_behavior) = ranged_behavior {
                        let distance_to_player = transform.translation.distance(player_pos);
                        let direction = (player_pos - transform.translation).normalize();

                        let distance_difference =
                            distance_to_player - ranged_behavior.preferred_distance;

                        if distance_difference.abs() > ranged_behavior.tolerance {
                            if distance_difference > 0.0 {
                                movement = direction.truncate() * enemy.speed as f32;
                            } else {
                                movement = -direction.truncate() * enemy.speed as f32;
                            }
                        } else if distance_difference.abs() > ranged_behavior.tolerance * 0.5 {
                            movement = direction.truncate() * (distance_difference * 0.1);
                        }
                    } else {
                        *state = EnemyState::Pursuing;
                    }
                }
                EnemyState::Retreating => {
                    let dir = (transform.translation - player_pos).normalize();
                    movement = dir.truncate() * enemy.speed as f32;
                }
            }

            let collision_resolution =
                calculate_collision_resolution(entity, transform.translation, &enemy_positions);
            let final_movement = movement + collision_resolution;

            apply_movement(&mut transform.translation, final_movement, LAYER2);
        }
    }
}

fn calculate_collision_resolution(
    current_entity: Entity,
    current_pos: Vec3,
    enemy_positions: &Vec<(Entity, Vec3)>,
) -> Vec2 {
    let collision_radius = 30.0;
    let mut collision_resolution = Vec2::ZERO;

    for (other_entity, other_pos) in enemy_positions {
        if *other_entity != current_entity {
            let diff = current_pos - *other_pos;
            let distance = diff.length();

            if distance < collision_radius {
                if distance > 0.0 {
                    collision_resolution +=
                        diff.truncate().normalize() * (collision_radius - distance);
                } else {
                    let random_angle = rand::random::<f32>() * std::f32::consts::TAU;
                    collision_resolution +=
                        Vec2::new(random_angle.cos(), random_angle.sin()) * collision_radius;
                }
            }
        }
    }

    collision_resolution
}

pub fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    handle: Res<GlobalTextureAtlas>,
    player_query: Query<(&Transform, &Health), With<Player>>,
    enemy_query: Query<(), With<Enemy>>,
    mut indicator_query: Query<(Entity, &mut SpawnIndicator, &mut Sprite)>,
    mut wave: ResMut<Wave>,
    game_mode: Res<GameMode>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !enemy_query.is_empty() {
        return;
    }

    if player_query.is_empty() {
        return;
    }

    let (player_transform, health) = player_query.single();
    if health.0 == 0 {
        return;
    }

    for (entity, mut indicator, mut sprite) in indicator_query.iter_mut() {
        indicator.timer.tick(time.delta());

        let alpha = (indicator.timer.elapsed_secs() * 5.0).sin().abs();
        sprite.color.set_alpha(alpha);

        if indicator.timer.finished() {
            commands.entity(entity).despawn();

            let mut difficulty_multiplier = calculate_difficulty_multiplier(wave.number);

            if *game_mode == GameMode::Forever && is_boss_wave(wave.number) {
                difficulty_multiplier *= 1.5;
            }

            let enemy_base = if is_boss_wave(wave.number) {
                create_midgame_boss_enemy()
            } else {
                select_enemy_type(wave.number)()
            };

            let health = (enemy_base.health as f32 * difficulty_multiplier) as u32;
            let speed = (enemy_base.speed as f32 * difficulty_multiplier) as u32;
            let damage = (enemy_base.damage as f32 * difficulty_multiplier) as u32;
            let xp = (enemy_base.xp as f32 * difficulty_multiplier) as u32;

            let enemy = enemy_base.with_stats(health, speed, damage, xp);

            enemy.spawn(&mut commands, indicator.spawn_position, &handle);
        }
    }

    if indicator_query.is_empty() {
        wave.number += 1;

        if *game_mode == GameMode::Normal && wave.number > 10 {
            next_state.set(GameState::Win);
            return;
        }

        let player_pos = player_transform.translation.truncate();
        let num_enemies = calculate_enemies_for_wave(wave.number);

        for _ in 0..num_enemies {
            let (x, y) = get_random_position_around(player_pos, 250.0..1000.0);
            let mut position = Vec3::new(x, y, LAYER2);
            clamp_position(&mut position);

            commands.spawn((
                Name::new("SpawnIndicator"),
                Sprite {
                    image: handle.image.clone().unwrap(),
                    texture_atlas: Some(TextureAtlas {
                        layout: handle.layout_16x16.clone().unwrap(),
                        index: 160,
                    }),
                    color: Color::srgba(1.0, 1.0, 1.0, 1.0),
                    ..default()
                },
                Transform::from_translation(position).with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                SpawnIndicator {
                    timer: Timer::from_seconds(2.5, TimerMode::Once),
                    spawn_position: position,
                },
                InGameEntity,
            ));
        }
    }
}

fn calculate_enemy_distribution(wave: u32) -> Vec<(fn() -> EnemyBuilder, f32)> {
    let mut distributions = Vec::new();

    let basic_weight = 1.0 / (1.0 + (wave as f32 * 0.1));
    distributions.push((create_basic_enemy as fn() -> EnemyBuilder, basic_weight));

    if wave >= 2 {
        let charging_weight = (wave as f32 * 0.15).min(0.8);
        distributions.push((
            create_charging_enemy as fn() -> EnemyBuilder,
            charging_weight,
        ));
    }

    if wave >= 3 {
        let splitter_weight = (wave as f32 * 0.06).min(0.35);
        distributions.push((
            create_splitting_enemy as fn() -> EnemyBuilder,
            splitter_weight,
        ));
    }

    if wave >= 4 {
        let trail_weight = (wave as f32 * 0.1).min(0.6);
        distributions.push((create_trail_enemy as fn() -> EnemyBuilder, trail_weight));
    }

    if wave >= 5 {
        let shooter_weight = (wave as f32 * 0.08).min(0.5);
        distributions.push((create_shooter_enemy as fn() -> EnemyBuilder, shooter_weight));
    }

    if wave >= 6 {
        let bomber_weight = (wave as f32 * 0.07).min(0.4);
        distributions.push((create_bomber_enemy as fn() -> EnemyBuilder, bomber_weight));
    }

    if wave >= 7 {
        let gurgle_weight = (wave as f32 * 0.06).min(0.3);
        distributions.push((create_gurgle_enemy as fn() -> EnemyBuilder, gurgle_weight));
    }

    distributions
}

fn select_enemy_type(wave: u32) -> fn() -> EnemyBuilder {
    let distributions = calculate_enemy_distribution(wave);
    let total_weight: f32 = distributions.iter().map(|(_, weight)| weight).sum();
    let mut rng = rand::thread_rng();
    let random_value = rng.gen::<f32>() * total_weight;

    let mut cumulative_weight = 0.0;
    for (enemy_type, weight) in distributions {
        cumulative_weight += weight;
        if random_value <= cumulative_weight {
            return enemy_type;
        }
    }

    create_basic_enemy
}

fn calculate_enemies_for_wave(wave_number: u32) -> u32 {
    if is_boss_wave(wave_number) {
        let base_boss_count = 3;
        let additional_bosses = if wave_number <= 10 {
            0
        } else {
            ((wave_number - 10) / 10) as u32
        };

        base_boss_count + additional_bosses
    } else {
        let base = match wave_number {
            1..=3 => 15 + wave_number * 3,
            4..=6 => 25 + wave_number * 4,
            7..=9 => 45 + wave_number * 5,
            _ => {
                let scaling = if wave_number <= 15 {
                    90 + (wave_number - 9) * 8
                } else {
                    140 + (wave_number - 15) * 12
                };
                scaling
            }
        };

        base + (random::<u32>() % 15)
    }
}

pub fn calculate_difficulty_multiplier(wave_number: u32) -> f32 {
    if wave_number <= 30 {
        1.0 + (wave_number as f32 / 10.0) * 0.1
    } else {
        let base_multiplier = 1.0 + (30.0 / 10.0 * 0.1);
        let additional_multiplier = ((wave_number - 30) as f32 / 5.0) * 0.1;
        base_multiplier + additional_multiplier
    }
}

fn is_boss_wave(wave_number: u32) -> bool {
    wave_number % 10 == 0 && wave_number >= 10
}

pub fn update_spawn_indicators(
    time: Res<Time>,
    mut indicator_query: Query<(&mut SpawnIndicator, &mut Sprite)>,
) {
    for (mut indicator, mut sprite) in indicator_query.iter_mut() {
        indicator.timer.tick(time.delta());

        let alpha = (indicator.timer.elapsed_secs() * 5.0).sin().abs();
        sprite.color.set_alpha(alpha);
    }
}

pub fn handle_trail_abilities(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&Transform, &mut TrailAbility), With<Enemy>>,
) {
    for (transform, mut trail_ability) in query.iter_mut() {
        trail_ability.timer.tick(time.delta());
        if trail_ability.timer.just_finished() {
            let current_position = transform.translation;
            if let Some(last_position) = trail_ability.last_position {
                spawn_trail_segment(
                    &mut commands,
                    last_position,
                    current_position,
                    trail_ability.damage,
                    trail_ability.trail_radius,
                    trail_ability.trail_duration,
                );
            }
            trail_ability.last_position = Some(current_position);
        }
    }
}

pub fn handle_enemy_bullet_player_collision(
    mut commands: Commands,
    player_query: Query<&Transform, (With<Player>, Without<InvincibilityEffect>)>,
    bullet_query: Query<(Entity, &Transform, &BulletStats), With<EnemyBullet>>,
    mut ev_player_damaged: EventWriter<PlayerDamagedEvent>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (bullet_entity, bullet_transform, bullet_stats) in bullet_query.iter() {
            let distance = player_transform
                .translation
                .distance(bullet_transform.translation);

            if distance < 30.0 {
                ev_player_damaged.send(PlayerDamagedEvent {
                    damage: bullet_stats.damage,
                });
                commands.entity(bullet_entity).try_despawn();
            }
        }
    }
}

pub fn handle_hit_flash(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Sprite, &mut HitFlash)>,
) {
    for (entity, mut sprite, mut flash) in query.iter_mut() {
        flash.0.tick(time.delta());
        sprite.color = Color::srgba(1.0, 0.0, 0.0, 1.0);

        if flash.0.just_finished() {
            sprite.color = Color::WHITE;
            commands.entity(entity).remove::<HitFlash>();
        }
    }
}

pub fn handle_shooting_abilities(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_query: Query<(&Transform, &mut ShootingAbility, Option<&GurgleEnemy>)>,
    player_query: Query<&Transform, With<Player>>,
    handle: Res<GlobalTextureAtlas>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (transform, mut shooting, gurgle_marker) in enemy_query.iter_mut() {
            let distance = transform.translation.distance(player_transform.translation);
            shooting.in_range = distance <= shooting.range;

            shooting.shoot_timer.tick(time.delta());

            if shooting.in_range && shooting.shoot_timer.just_finished() {
                let direction = (player_transform.translation - transform.translation).normalize();

                let is_exploding = gurgle_marker.is_some();

                spawn_enemy_bullets(
                    &mut commands,
                    transform.translation,
                    direction,
                    shooting.bullets_per_shot,
                    &handle,
                    is_exploding,
                    shooting.bullet_speed,
                    shooting.bullet_damage,
                );
            }
        }
    }
}

pub fn handle_exploding_bullets(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &ExplodingBullet), With<EnemyBullet>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (bullet_entity, bullet_transform, exploding_bullet) in bullet_query.iter() {
            let distance_to_player = bullet_transform
                .translation
                .distance(player_transform.translation);

            if distance_to_player <= 30.0 {
                spawn_explosion(
                    &mut commands,
                    bullet_transform.translation,
                    exploding_bullet.radius,
                    exploding_bullet.damage,
                );
                commands.entity(bullet_entity).try_despawn();
            }

            if bullet_transform.translation.y <= -WH + 20.0 {
                spawn_explosion(
                    &mut commands,
                    bullet_transform.translation,
                    exploding_bullet.radius,
                    exploding_bullet.damage,
                );
                commands.entity(bullet_entity).try_despawn();
            }
        }
    }
}

pub fn handle_charge_abilities(
    time: Res<Time>,
    mut enemy_query: Query<(Entity, &mut Transform, &mut ChargeAbility, &Enemy)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let enemy_positions: Vec<(Entity, Vec3)> = enemy_query
            .iter()
            .map(|(entity, transform, _, _)| (entity, transform.translation))
            .collect();

        for (entity, mut transform, mut charge, enemy) in enemy_query.iter_mut() {
            charge.charge_timer.tick(time.delta());

            let mut movement = Vec2::ZERO;

            match charge.state {
                ChargeState::Approaching => {
                    if transform.translation.distance(player_transform.translation)
                        <= charge.charge_distance as f32
                    {
                        charge.state = ChargeState::Preparing;
                        charge.charge_timer = Timer::from_seconds(1.5, TimerMode::Once);
                    } else {
                        let direction =
                            (player_transform.translation - transform.translation).normalize();
                        movement = direction.truncate() * enemy.speed as f32;
                    }
                }
                ChargeState::Preparing => {
                    if charge.charge_timer.just_finished() {
                        charge.state = ChargeState::Charging;
                        charge.charge_timer = Timer::from_seconds(0.5, TimerMode::Once);
                        charge.target_position = Some(player_transform.translation.truncate());
                    }
                }
                ChargeState::Charging => {
                    if charge.charge_timer.just_finished() {
                        charge.state = ChargeState::Cooldown;
                        charge.charge_timer =
                            Timer::from_seconds(charge.cooldown_duration, TimerMode::Once);
                        charge.target_position = None;
                    } else if let Some(target) = charge.target_position {
                        let direction = (target.extend(0.0) - transform.translation).normalize();
                        movement = direction.truncate() * charge.charge_speed as f32;

                        let shake_amount = 2.0;
                        let shake_offset = Vec2::new(
                            rand::random::<f32>() * shake_amount - shake_amount / 2.0,
                            rand::random::<f32>() * shake_amount - shake_amount / 2.0,
                        );
                        transform.translation += shake_offset.extend(0.0);
                    }
                }
                ChargeState::Cooldown => {
                    if charge.charge_timer.just_finished() {
                        charge.state = ChargeState::Approaching;
                    } else {
                        let direction =
                            (player_transform.translation - transform.translation).normalize();
                        movement = direction.truncate() * enemy.speed as f32;

                        let shake_amount = 1.0;
                        let shake_offset = Vec2::new(
                            rand::random::<f32>() * shake_amount - shake_amount / 2.0,
                            rand::random::<f32>() * shake_amount - shake_amount / 2.0,
                        );
                        transform.translation += shake_offset.extend(0.0);
                    }
                }
            }

            let collision_resolution =
                calculate_collision_resolution(entity, transform.translation, &enemy_positions);
            let final_movement = movement + collision_resolution;

            apply_movement(&mut transform.translation, final_movement, LAYER2);
        }
    }
}

pub fn handle_charge_enemy_flash(
    mut enemy_query: Query<(&ChargeAbility, &mut Sprite, &OriginalEnemyColor), Without<HitFlash>>,
    time: Res<Time>,
) {
    for (charge, mut sprite, original_color) in enemy_query.iter_mut() {
        match charge.state {
            ChargeState::Preparing => {
                let flash_rate = 18.0;
                let flash_amount = (time.elapsed_secs() * flash_rate).sin() * 0.5 + 0.5;

                sprite.color = Color::srgba(0.3, 0.7, 1.0, flash_amount);
            }
            _ => {
                sprite.color = original_color.0;
            }
        }
    }
}

pub fn handle_enemy_death(
    mut commands: Commands,
    mut enemy_query: Query<(
        Entity,
        &Enemy,
        &Transform,
        Option<&ExplosionAbility>,
        Option<&LootPool>,
    )>,
    mut ev_player_damaged: EventWriter<PlayerDamagedEvent>,
    mut level: ResMut<Level>,
    handle: Res<GlobalTextureAtlas>,
    player_query: Query<(&Transform, Option<&InvincibilityEffect>), With<Player>>,
    mut ev_level_up: EventWriter<PlayerLevelingUpEvent>,
    mut ew: EventWriter<AudioEvent>,
) {
    if let Ok((player_transform, is_invincible)) = player_query.get_single() {
        for (entity, enemy, transform, explosion_ability, loot_pool) in enemy_query.iter_mut() {
            if enemy.health == 0 {
                ew.send(AudioEvent::Kill);
                if let Some(explosion) = explosion_ability {
                    spawn_explosion(
                        &mut commands,
                        transform.translation,
                        explosion.explosion_radius,
                        explosion.explosion_damage,
                    );
                    let distance = player_transform.translation.distance(transform.translation);
                    if distance <= explosion.explosion_radius && is_invincible.is_none() {
                        ev_player_damaged.send(PlayerDamagedEvent {
                            damage: explosion.explosion_damage,
                        });
                    }
                }

                if let Some(loot_pool) = loot_pool {
                    let loot_defs = loot_pool.get_random_loots();
                    for loot_def in loot_defs {
                        (loot_def.spawn_fn)(
                            &mut commands,
                            transform,
                            handle.image.clone(),
                            handle.layout_16x16.clone(),
                            loot_def.stat_range,
                            loot_def.value,
                        );
                    }
                }

                if level.add_xp(enemy.xp) {
                    ev_level_up.send(PlayerLevelingUpEvent {
                        new_level: level.level(),
                    });
                }

                commands
                    .entity(entity)
                    .insert(DeathEffect::default())
                    .remove::<Enemy>();
            }
        }
    }
}

pub fn handle_death_effect(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Sprite, &mut DeathEffect)>,
) {
    for (entity, mut transform, mut sprite, mut effect) in query.iter_mut() {
        effect.timer.tick(time.delta());

        let progress = effect.timer.fraction();

        sprite.color.set_alpha(1.0 - progress);

        let scale = effect.initial_scale * (1.0 + progress * 0.5);
        transform.scale = scale;

        if effect.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_trail_segment(
    commands: &mut Commands,
    start: Vec3,
    end: Vec3,
    damage: u32,
    radius: f32,
    duration: f32,
) {
    let direction = (end - start).normalize();
    let length = (end - start).length();
    let angle = direction.y.atan2(direction.x);
    let center_pos = start + (end - start) / 2.0;

    commands.spawn((
        Name::new("TrailSegment"),
        TrailSegment {
            start,
            end,
            timer: Timer::from_seconds(duration, TimerMode::Once),
            width: radius * 2.0,
        },
        Trail { damage, radius },
        HasLifespan::new(Duration::from_secs_f32(duration)),
        Sprite {
            color: Color::srgba(0.0, 0.8, 0.0, 0.6),
            custom_size: Some(Vec2::new(length, radius * 2.0)),
            ..default()
        },
        Transform::from_translation(center_pos).with_rotation(Quat::from_rotation_z(angle)),
        InGameEntity,
    ));
}

pub fn update_trail_segments(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TrailSegment, &mut Sprite)>,
) {
    for (entity, mut segment, mut sprite) in query.iter_mut() {
        segment.timer.tick(time.delta());

        let progress = segment.timer.elapsed_secs() / segment.timer.duration().as_secs_f32();

        if progress >= 1.0 {
            commands.entity(entity).despawn();
        } else {
            let alpha = (1.0 - progress) * 0.6;

            let base_color = Color::srgba(progress * 0.8, (1.0 - progress) * 0.8, 0.0, alpha);

            sprite.color = base_color;

            if let Some(size) = &mut sprite.custom_size {
                size.x = segment.width * (1.0 - progress * 0.3);
            }
        }
    }
}

pub fn spawn_explosion(commands: &mut Commands, position: Vec3, radius: f32, damage: u32) {
    commands.spawn((
        Name::new("Explosion"),
        Sprite {
            color: Color::srgba(1.0, 0.5, 0.0, 0.5),
            custom_size: Some(Vec2::new(radius * 2.0, radius * 2.0)),
            ..default()
        },
        Transform::from_translation(position),
        Explosion {
            radius,
            damage,
            timer: Timer::from_seconds(0.3, TimerMode::Once),
        },
    ));
}

pub fn update_enemy_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut bullet_query: Query<
        (
            Entity,
            &mut Transform,
            &BulletDirection,
            &BulletStats,
            Option<&ExplodingBullet>,
        ),
        With<EnemyBullet>,
    >,
) {
    for (entity, mut transform, direction, stats, exploding) in bullet_query.iter_mut() {
        transform.translation += direction.0 * stats.speed as f32 * time.delta_secs();

        if transform.translation.x.abs() > WW || transform.translation.y.abs() > WH {
            if let Some(exploding_bullet) = exploding {
                spawn_explosion(
                    &mut commands,
                    transform.translation,
                    exploding_bullet.radius,
                    exploding_bullet.damage,
                );
            }
            commands.entity(entity).try_despawn();
        }
    }
}

fn spawn_enemy_bullets(
    commands: &mut Commands,
    enemy_pos: Vec3,
    direction: Vec3,
    num_bullets: usize,
    handle: &GlobalTextureAtlas,
    is_exploding: bool,
    bullet_speed: u32,
    bullet_damage: u32,
) {
    for _ in 0..num_bullets {
        let spread = Vec3::new(
            rand::random::<f32>() * 0.2 - 0.1,
            rand::random::<f32>() * 0.2 - 0.1,
            0.0,
        );
        let bullet_direction = (direction + spread).normalize();

        let sprite_index = if is_exploding { 89 } else { 88 };

        let mut bullet_entity = commands.spawn((
            Name::new("Enemy Bullet"),
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout_16x16.clone().unwrap(),
                    index: sprite_index,
                }),
                ..default()
            },
            Transform::from_translation(enemy_pos).with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            EnemyBullet,
            BulletDirection(bullet_direction),
            BulletStats {
                speed: bullet_speed,
                damage: bullet_damage,
                lifespan: BULLET_TIME_SECS,
            },
        ));

        if is_exploding {
            bullet_entity.insert(ExplodingBullet {
                radius: 135.0,
                damage: 30,
            });
        }
    }
}

pub fn handle_explosion_player_collision(
    explosion_query: Query<(&Transform, &Explosion)>,
    player_query: Query<&Transform, (With<Player>, Without<InvincibilityEffect>)>,
    mut ev_player_damaged: EventWriter<PlayerDamagedEvent>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (explosion_transform, explosion) in explosion_query.iter() {
            let distance = player_transform
                .translation
                .distance(explosion_transform.translation);

            if distance <= explosion.radius {
                ev_player_damaged.send(PlayerDamagedEvent {
                    damage: explosion.damage,
                });
            }
        }
    }
}

pub fn handle_explosions(
    mut commands: Commands,
    time: Res<Time>,
    mut explosion_query: Query<(Entity, &mut Explosion, &mut Sprite)>,
) {
    for (entity, mut explosion, mut sprite) in explosion_query.iter_mut() {
        explosion.timer.tick(time.delta());

        let alpha =
            0.5 * (1.0 - explosion.timer.elapsed_secs() / explosion.timer.duration().as_secs_f32());
        sprite.color = Color::srgba(1.0, 0.5, 0.0, alpha);

        if explosion.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn handle_ranged_movement(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<
        (&Enemy, &mut Transform, &mut EnemyState, &RangedBehavior),
        Without<Player>,
    >,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation;

        for (enemy, mut transform, mut state, range_behavior) in enemy_query.iter_mut() {
            let distance_to_player = transform.translation.distance(player_pos);
            let direction = (player_pos - transform.translation).normalize();

            let distance_difference = distance_to_player - range_behavior.preferred_distance;

            if distance_difference.abs() > range_behavior.tolerance {
                if distance_difference > 0.0 {
                    *state = EnemyState::Pursuing;
                    let movement = direction.truncate() * enemy.speed as f32;
                    apply_movement(&mut transform.translation, movement, LAYER2);
                } else {
                    *state = EnemyState::Retreating;
                    let movement = -direction.truncate() * enemy.speed as f32;
                    apply_movement(&mut transform.translation, movement, LAYER2);
                }
            } else {
                *state = EnemyState::MaintainingDistance;

                if distance_difference.abs() > range_behavior.tolerance * 0.5 {
                    let movement = direction.truncate() * (distance_difference * 0.1);
                    apply_movement(&mut transform.translation, movement, LAYER2);
                }
            }
        }
    }
}

pub fn handle_enemy_splitting(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform, &Enemy, &SplitAbility)>,
    handle: Res<GlobalTextureAtlas>,
) {
    for (_, transform, enemy, split_ability) in enemy_query.iter() {
        if enemy.health == 0 && split_ability.splits_remaining > 0 {
            let num_spawns = match split_ability.splits_remaining {
                3 => 4,
                2 => 2,
                1 => 1,
                _ => 0,
            };

            for i in 0..num_spawns {
                let angle = (i as f32 / num_spawns as f32) * 2.0 * std::f32::consts::PI;
                let offset = Vec2::new(angle.cos(), angle.sin()) * 30.0;
                let new_pos = transform.translation + Vec3::new(offset.x, offset.y, 0.0);

                let enemy_builder = EnemyBuilder::new()
                    .with_stats(enemy.health + 20, enemy.speed, enemy.damage, enemy.xp / 2)
                    .with_sprite(56, (16, 16))
                    .with_splitting(split_ability.splits_remaining - 1);

                enemy_builder.spawn(&mut commands, new_pos, &handle);
            }
        }
    }
}

pub fn handle_summoning_abilities(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&Transform, &mut SummoningAbility), With<Enemy>>,
    handle: Res<GlobalTextureAtlas>,
) {
    for (transform, mut summoning_ability) in query.iter_mut() {
        summoning_ability.timer.tick(time.delta());
        if summoning_ability.timer.just_finished() {
            let num_minions = rand::thread_rng()
                .gen_range(summoning_ability.min_minions..=summoning_ability.max_minions);
            let spread_radius_min = 200.0;
            let spread_radius_max = 1000.0;
            let angle_step = 2.0 * std::f32::consts::PI / num_minions as f32;
            for i in 0..num_minions {
                let angle = angle_step * i as f32;
                let radius = rand::thread_rng().gen_range(spread_radius_min..spread_radius_max);

                let offset_x = angle.cos() * radius;
                let offset_y = angle.sin() * radius;

                let mut position = transform.translation + Vec3::new(offset_x, offset_y, 0.0);
                clamp_position(&mut position);

                let enemy = match rand::random::<f32>() {
                    x if x < 0.03 => create_splitting_enemy(),
                    x if x < 0.2 => create_basic_enemy(),
                    x if x < 0.4 => create_trail_enemy(),
                    x if x < 0.65 => create_charging_enemy(),
                    x if x < 0.8 => create_shooter_enemy(),
                    x if x < 0.95 => create_bomber_enemy(),
                    _ => create_gurgle_enemy(),
                };
                enemy.spawn(&mut commands, position, &handle);
            }
        }
    }
}
