use super::{components::*, presets::*};
use crate::{
    configs::*,
    enemy::EnemyBuilder,
    gun::{BulletDirection, BulletStats, HasLifespan},
    loot::LootPool,
    player::{Health, InvincibilityEffect, Player, PlayerDamagedEvent, PlayerLevelingUpEvent},
    resources::{GlobalTextureAtlas, Level, Wave},
    utils::{
        apply_movement, calculate_enemies_for_wave, clamp_position, get_random_position_around,
        InGameEntity,
    },
};
use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

pub fn update_enemy_movement(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<
        (&Enemy, &mut Transform, &mut EnemyState),
        (
            Without<Player>,
            Without<ChargeAbility>,
            Without<RangedBehavior>,
        ),
    >,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation;

        for (enemy, mut transform, mut state) in enemy_query.iter_mut() {
            match &mut *state {
                EnemyState::Wandering {
                    direction,
                    timer,
                    has_been_aggroed,
                } => {
                    timer.tick(time.delta());

                    let distance_to_player = transform.translation.distance(player_pos);
                    if distance_to_player < 300.0 {
                        *has_been_aggroed = true;
                        *state = EnemyState::Pursuing;
                    } else {
                        if timer.just_finished() {
                            let angle = rand::thread_rng().gen_range(0.0..std::f32::consts::TAU);
                            *direction = Vec2::new(angle.cos(), angle.sin());
                        }

                        let movement = *direction * enemy.speed as f32 * 0.5;
                        apply_movement(&mut transform.translation, movement, LAYER1);
                    }
                }
                EnemyState::Pursuing => {
                    let direction = (player_pos - transform.translation).normalize();
                    let movement = direction.truncate() * enemy.speed as f32;
                    apply_movement(&mut transform.translation, movement, LAYER1);

                    let distance_to_player = transform.translation.distance(player_pos);
                    if distance_to_player > 400.0 {
                        match state.as_ref() {
                            EnemyState::Wandering {
                                has_been_aggroed, ..
                            } if !has_been_aggroed => {
                                *state = EnemyState::Wandering {
                                    direction: Vec2::new(1.0, 0.0),
                                    timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                                    has_been_aggroed: false,
                                };
                            }
                            _ => continue,
                        }
                    }
                }
                EnemyState::Retreating | EnemyState::MaintainingDistance => {
                    *state = EnemyState::Pursuing;
                }
            }
        }
    }
}

pub fn spawn_enemies(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    player_query: Query<(&Transform, &Health), With<Player>>,
    enemy_query: Query<(), With<Enemy>>,
    mut wave: ResMut<Wave>,
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

    wave.number += 1;
    let player_pos = player_transform.translation.truncate();

    let num_enemies = calculate_enemies_for_wave(wave.number);

    for _ in 0..num_enemies {
        let (x, y) = get_random_position_around(player_pos, 200.0..500.0);
        let mut position = Vec3::new(x, y, LAYER1);
        clamp_position(&mut position);

        let enemy = match wave.number {
            1..=2 => create_basic_enemy(),
            3..=4 => {
                if rand::random::<f32>() < 0.5 {
                    create_trail_enemy()
                } else {
                    create_charging_enemy()
                }
            }
            5..=7 => match rand::random::<f32>() {
                x if x < 0.3 => create_shooter_enemy(),
                x if x < 0.6 => create_bomber_enemy(),
                x if x < 0.8 => create_splitting_enemy(),
                _ => create_trail_enemy(),
            },
            8..=9 => create_gurgle_enemy(),
            _ => {
                if wave.number % 10 == 0 {
                    create_midgame_boss_enemy()
                } else {
                    match rand::random::<f32>() {
                        x if x < 0.3 => create_charging_enemy(),
                        x if x < 0.4 => create_trail_enemy(),
                        x if x < 0.5 => create_splitting_enemy(),
                        x if x < 0.7 => create_shooter_enemy(),
                        x if x < 0.9 => create_bomber_enemy(),
                        _ => create_gurgle_enemy(),
                    }
                }
            }
        };

        enemy.spawn(&mut commands, position, &handle);
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
    mut enemy_query: Query<(&mut Transform, &mut ChargeAbility, &Enemy)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (mut transform, mut charge, enemy) in enemy_query.iter_mut() {
            charge.charge_timer.tick(time.delta());

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
                        let movement = direction.truncate() * enemy.speed as f32;
                        apply_movement(&mut transform.translation, movement, LAYER1);
                    }
                }
                ChargeState::Preparing => {
                    if charge.charge_timer.just_finished() {
                        charge.state = ChargeState::Charging;
                        charge.charge_timer = Timer::from_seconds(2.0, TimerMode::Once);
                        charge.target_position = Some(player_transform.translation.truncate());
                    }
                }
                ChargeState::Charging => {
                    if charge.charge_timer.just_finished() {
                        charge.state = ChargeState::CoolingDown;
                        charge.charge_timer = Timer::from_seconds(1.5, TimerMode::Once);
                        charge.target_position = None;
                    } else if let Some(target) = charge.target_position {
                        let direction = (target.extend(0.0) - transform.translation).normalize();
                        let movement = direction.truncate() * charge.charge_speed as f32;
                        apply_movement(&mut transform.translation, movement, LAYER1);
                    }
                }
                ChargeState::CoolingDown => {
                    if charge.charge_timer.just_finished() {
                        charge.state = ChargeState::Approaching;
                    }
                }
            }
        }
    }
}

pub fn handle_charge_enemy_flash(
    mut enemy_query: Query<(&ChargeAbility, &mut Sprite, &OriginalEnemyColor)>,
    time: Res<Time>,
) {
    for (charge, mut sprite, original_color) in enemy_query.iter_mut() {
        match charge.state {
            ChargeState::Preparing => {
                let flash_rate = 18.0;
                let flash_amount = (time.elapsed_secs() * flash_rate).sin() * 0.5 + 0.5;

                sprite.color = Color::srgba(1.0, 0.0, 0.0, flash_amount);
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
) {
    if let Ok((player_transform, is_invincible)) = player_query.get_single() {
        for (entity, enemy, transform, explosion_ability, loot_pool) in enemy_query.iter_mut() {
            if enemy.health == 0 {
                if let Some(explosion) = explosion_ability {
                    spawn_explosion(
                        &mut commands,
                        transform.translation,
                        explosion.explosion_radius,
                        explosion.explosion_damage,
                    );
                    let distance = player_transform.translation.distance(transform.translation);
                    if distance <= explosion.explosion_radius && is_invincible.is_none() {
                        println!("send explosion event");
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
                        );
                    }
                }

                if level.add_xp(enemy.xp) {
                    ev_level_up.send(PlayerLevelingUpEvent {
                        new_level: level.level(),
                    });
                }

                commands.entity(entity).despawn();
            }
        }
    }
}

// fn spawn_trail(commands: &mut Commands, position: Vec3, damage: u32, radius: f32) {
//     commands.spawn((
//         Name::new("Trail"),
//         Sprite {
//             color: Color::srgba(0.0, 0.8, 0.0, 0.5),
//             custom_size: Some(Vec2::new(20.0, 20.0)),
//             ..default()
//         },
//         Transform::from_translation(position),
//         Trail { damage, radius },
//     ));
// }

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

        let mut bullet_entity = commands.spawn((
            Name::new("Enemy Bullet"),
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout_16x16.clone().unwrap(),
                    index: 81,
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
                radius: 100.0,
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
                    apply_movement(&mut transform.translation, movement, LAYER1);
                } else {
                    *state = EnemyState::Retreating;
                    let movement = -direction.truncate() * enemy.speed as f32;
                    apply_movement(&mut transform.translation, movement, LAYER1);
                }
            } else {
                *state = EnemyState::MaintainingDistance;

                if distance_difference.abs() > range_behavior.tolerance * 0.5 {
                    let movement = direction.truncate() * (distance_difference * 0.1);
                    apply_movement(&mut transform.translation, movement, LAYER1);
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
            for _ in 0..num_minions {
                let offset = Vec2::new(
                    rand::random::<f32>() * 100.0 - 50.0,
                    rand::random::<f32>() * 100.0 - 50.0,
                );
                let position = transform.translation + Vec3::new(offset.x, offset.y, 0.0);
                let enemy = match rand::random::<f32>() {
                    x if x < 0.2 => create_basic_enemy(),
                    x if x < 0.4 => create_trail_enemy(),
                    x if x < 0.6 => create_shooter_enemy(),
                    x if x < 0.8 => create_charging_enemy(),
                    _ => create_bomber_enemy(),
                };
                enemy.spawn(&mut commands, position, &handle);
            }
        }
    }
}
