use super::{components::*, presets::*};
use crate::{
    configs::*,
    gun::{BulletDirection, BulletStats},
    loot::LootPool,
    player::{Health, InvincibilityEffect, Player, PlayerDamagedEvent, PlayerLevelingUpEvent},
    resources::{GlobalTextureAtlas, Level, Wave},
    utils::{calculate_enemies_for_wave, clamp_position, get_random_position_around},
};
use bevy::prelude::*;
use rand::Rng;

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
                EnemyState::Wandering { direction, timer } => {
                    timer.tick(time.delta());

                    let distance_to_player = transform.translation.distance(player_pos);
                    if distance_to_player < 300.0 {
                        *state = EnemyState::Pursuing;
                    } else {
                        if timer.just_finished() {
                            let angle = rand::thread_rng().gen_range(0.0..std::f32::consts::TAU);
                            *direction = Vec2::new(angle.cos(), angle.sin());
                        }

                        transform.translation +=
                            Vec3::new(direction.x, direction.y, LAYER1) * enemy.speed as f32 * 0.5;
                    }
                }
                EnemyState::Pursuing => {
                    let direction = (player_pos - transform.translation).normalize();
                    transform.translation += direction * enemy.speed as f32;

                    let distance_to_player = transform.translation.distance(player_pos);
                    if distance_to_player > 400.0 {
                        *state = EnemyState::Wandering {
                            direction: Vec2::new(1.0, 0.0),
                            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                        };
                    }
                }
                EnemyState::Retreating | EnemyState::MaintainingDistance => {
                    *state = EnemyState::Pursuing;
                }
            }

            transform.translation.x = transform.translation.x.clamp(-WW, WW);
            transform.translation.y = transform.translation.y.clamp(-WH, WH);
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

    println!("wave number: {:?}", wave.number);
    let player_pos = player_transform.translation.truncate();

    let num_enemies = calculate_enemies_for_wave(wave.number);

    for _ in 0..num_enemies {
        let (x, y) = get_random_position_around(player_pos, 300.0..800.0);
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
                x if x < 0.4 => create_shooter_enemy(),
                x if x < 0.7 => create_bomber_enemy(),
                _ => create_trail_enemy(),
            },
            _ => {
                if wave.number % 10 == 0 {
                    create_boss_enemy()
                } else {
                    match rand::random::<f32>() {
                        x if x < 0.3 => create_shooter_enemy(),
                        x if x < 0.6 => create_bomber_enemy(),
                        _ => create_trail_enemy(),
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
            spawn_trail(
                &mut commands,
                transform.translation,
                trail_ability.damage,
                trail_ability.trail_radius,
            );
        }
    }
}

pub fn handle_enemy_bullet_player_collision(
    mut commands: Commands,
    player_query: Query<&Transform, (With<Player>, Without<InvincibilityEffect>)>,
    bullet_query: Query<(Entity, &Transform), With<EnemyBullet>>,
    mut ev_player_damaged: EventWriter<PlayerDamagedEvent>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (bullet_entity, bullet_transform) in bullet_query.iter() {
            let distance = player_transform
                .translation
                .distance(bullet_transform.translation);

            if distance < 30.0 {
                ev_player_damaged.send(PlayerDamagedEvent { damage: 10 });
                commands.entity(bullet_entity).despawn();
            }
        }
    }
}

pub fn handle_shooting_abilities(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_query: Query<(&Transform, &mut ShootingAbility), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
    handle: Res<GlobalTextureAtlas>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (transform, mut shooting) in enemy_query.iter_mut() {
            let distance = transform.translation.distance(player_transform.translation);
            shooting.in_range = distance <= shooting.range;

            if shooting.in_range {
                shooting.shoot_timer.tick(time.delta());
                if shooting.shoot_timer.just_finished() {
                    let direction =
                        (player_transform.translation - transform.translation).normalize();

                    spawn_enemy_bullets(
                        &mut commands,
                        transform.translation,
                        direction,
                        shooting.bullets_per_shot,
                        &handle,
                    );
                    shooting.shoot_timer = Timer::from_seconds(0.5, TimerMode::Once);
                    shooting.reload_timer = Timer::from_seconds(2.0, TimerMode::Once);
                }
            } else {
                shooting.reload_timer.tick(time.delta());
                if shooting.reload_timer.finished() {
                    shooting.shoot_timer = Timer::from_seconds(0.5, TimerMode::Once);
                }
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
                        transform.translation += direction * enemy.speed as f32;
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
                        transform.translation += direction * charge.charge_speed as f32;
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
                            handle.layout.clone(),
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

fn spawn_trail(commands: &mut Commands, position: Vec3, damage: u32, radius: f32) {
    commands.spawn((
        Name::new("Trail"),
        Sprite {
            color: Color::srgba(0.0, 0.8, 0.0, 0.5),
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        Transform::from_translation(position),
        Trail { damage, radius },
    ));
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
        (Entity, &mut Transform, &BulletDirection, &BulletStats),
        With<EnemyBullet>,
    >,
) {
    for (entity, mut transform, direction, stats) in bullet_query.iter_mut() {
        transform.translation += direction.0 * stats.speed as f32 * time.delta_secs();

        if transform.translation.x.abs() > WW || transform.translation.y.abs() > WH {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_enemy_bullets(
    commands: &mut Commands,
    enemy_pos: Vec3,
    direction: Vec3,
    num_bullets: usize,
    handle: &GlobalTextureAtlas,
) {
    for _ in 0..num_bullets {
        let spread = Vec3::new(
            rand::random::<f32>() * 0.2 - 0.1,
            rand::random::<f32>() * 0.2 - 0.1,
            0.0,
        );
        let bullet_direction = (direction + spread).normalize();

        commands.spawn((
            Name::new("Enemy Bullet"),
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: 81,
                }),
                ..default()
            },
            Transform::from_translation(enemy_pos).with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            EnemyBullet,
            BulletDirection(bullet_direction),
        ));
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
                    transform.translation += direction * enemy.speed as f32;
                } else {
                    *state = EnemyState::Retreating;
                    transform.translation -= direction * enemy.speed as f32;
                }
            } else {
                *state = EnemyState::MaintainingDistance;

                if distance_difference.abs() > range_behavior.tolerance * 0.5 {
                    let adjustment = direction * (distance_difference * 0.1);
                    transform.translation += adjustment;
                }
            }

            transform.translation.x = transform.translation.x.clamp(-WW, WW);
            transform.translation.y = transform.translation.y.clamp(-WH, WH);
        }
    }
}
