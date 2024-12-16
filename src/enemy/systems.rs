use super::{components::EnemyBullet, *};
use crate::{
    configs::{LAYER1, SPAWN_RATE_PER_SECOND, SPRITE_SCALE_FACTOR, WH, WW},
    gun::{BulletDirection, BulletStats, HasLifespan},
    loot::{medium_enemies_bundle, strong_enemies_bundle, weak_enemies_bundle, LootPool},
    player::{InvincibilityEffect, Player, PlayerDamagedEvent, PlayerLevelingUpEvent},
    resources::{GlobalTextureAtlas, Level, Wave},
    utils::{
        calculate_enemies_per_wave, clamp_position, get_random_position_around, safe_subtract,
        InGameEntity,
    },
};
use bevy::prelude::*;
use rand::Rng;

use std::time::Duration;

pub fn spawn_enemies(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    player_query: Query<&Transform, With<Player>>,
    mut wave: ResMut<Wave>,
) {
    if wave.enemies_left == 0 || wave.requires_portal || wave.portal_spawned {
        return;
    }

    if wave.enemies_spawned >= wave.enemies_total || player_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation.truncate();
    for _ in 0..wave.enemies_left.min(SPAWN_RATE_PER_SECOND as u32) {
        let (x, y) = get_random_position_around(player_pos, 300.0..800.0);
        let enemy_type = EnemyType::Bomber {
            explosion_radius: 100.0,
            explosion_damage: 30,
            speed_multiplier: 1.5,
        };
        let _config = enemy_type.get_config();
        let loot_pool = match &enemy_type {
            EnemyType::Basic => weak_enemies_bundle(),
            EnemyType::LeaveTrail { .. } => medium_enemies_bundle(),
            EnemyType::Charge { .. } => medium_enemies_bundle(),
            EnemyType::Shooter { .. } => strong_enemies_bundle(),
            EnemyType::Bomber { .. } => strong_enemies_bundle(),
        };
        commands.spawn(EnemyBundle::new(
            enemy_type,
            Vec3::new(x.clamp(-WW, WW), y.clamp(-WH, WH), LAYER1),
            &handle,
            loot_pool,
        ));

        wave.enemies_spawned += 1;
    }
}

pub fn update_enemy_behavior(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Enemy, &mut Transform), Without<Player>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation;
    for (mut enemy, mut transform) in enemy_query.iter_mut() {
        let speed = enemy.speed;
        let movement = enemy.enemy_type.update_movement(
            transform.translation,
            player_pos,
            speed,
            time.delta(),
        );
        transform.translation += movement;
        enemy
            .enemy_type
            .apply(&mut commands, &transform, time.delta());
    }
}

pub fn despawn_dead_enemies(
    mut commands: Commands,
    enemy_query: Query<(&Enemy, Entity, &Transform, &LootPool), With<Enemy>>,
    mut wave: ResMut<Wave>,
    mut level: ResMut<Level>,
    handle: Res<GlobalTextureAtlas>,
    mut ew: EventWriter<PlayerLevelingUpEvent>,
    mut ew_bomber: EventWriter<BomberExplosionEvent>,
) {
    for (enemy, entity, transform, loot_pool) in enemy_query.iter() {
        if enemy.health == 0 {
            let loot_defs = loot_pool.get_random_loots();
            let mut rng = rand::thread_rng();

            for loot_def in loot_defs {
                let roll: f32 = rng.gen_range(0.0..1.0);
                if roll < loot_def.drop_chance {
                    (loot_def.spawn_fn)(
                        &mut commands,
                        transform,
                        handle.image.clone(),
                        handle.layout.clone(),
                        loot_def.stat_range,
                    );
                }
            }
            if let EnemyType::Bomber {
                explosion_radius,
                explosion_damage,
                ..
            } = enemy.enemy_type
            {
                ew_bomber.send(BomberExplosionEvent {
                    translation: transform.translation,
                    explosion_radius,
                    explosion_damage,
                });
            }
            commands.entity(entity).despawn();
            wave.enemies_left = safe_subtract(wave.enemies_left, 1);
            if level.add_xp(enemy.xp) {
                ew.send(PlayerLevelingUpEvent {
                    new_level: level.level(),
                });
            }
        }
    }

    if wave.enemies_left == 0 {
        {
            wave.number += 1;
            let new_wave_count = calculate_enemies_per_wave(wave.number);
            wave.enemies_total = new_wave_count;
            wave.enemies_left = new_wave_count;
            wave.enemies_spawned = 0;
        }
    }
}

pub fn handle_enemy_collision(mut enemy_query: Query<(Entity, &mut Transform, &Collider)>) {
    let mut combinations = enemy_query.iter_combinations_mut();
    while let Some(
        [(_entity_a, mut transform_a, collider_a), (_entity_b, mut transform_b, collider_b)],
    ) = combinations.fetch_next()
    {
        let distance = transform_a.translation.distance(transform_b.translation);
        let min_distance = (collider_a.radius + collider_b.radius) as f32;

        if distance < min_distance {
            let overlap = min_distance - distance;
            let direction = (transform_b.translation - transform_a.translation).normalize();

            transform_a.translation -= direction * overlap * 0.5;
            transform_b.translation += direction * overlap * 0.5;

            clamp_position(&mut transform_a.translation);
            clamp_position(&mut transform_b.translation);
        }
    }
}

pub fn handle_shooter_enemies(
    mut commands: Commands,
    mut enemy_query: Query<(&mut Enemy, &Transform, &mut EnemyType)>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
    handle: Res<GlobalTextureAtlas>,
) {
    if player_query.is_empty() {
        return;
    }
    let player_pos = player_query.single().translation;

    for (_enemy, transform, mut enemy_type) in enemy_query.iter_mut() {
        if let EnemyType::Shooter {
            ref mut shoot_timer,
            ref mut reload_timer,
            ref mut in_range,
            bullets_per_shot,
        } = enemy_type.as_mut()
        {
            let distance = transform.translation.distance(player_pos);
            *in_range = distance <= 300.0;

            if *in_range {
                shoot_timer.tick(time.delta());
                if shoot_timer.just_finished() {
                    spawn_enemy_bullets(
                        &mut commands,
                        transform.translation,
                        player_pos,
                        *bullets_per_shot,
                        &handle,
                    );
                    *shoot_timer = Timer::from_seconds(0.5, TimerMode::Once);
                    *reload_timer = Timer::from_seconds(2.0, TimerMode::Once);
                }
            } else {
                reload_timer.tick(time.delta());
                if reload_timer.finished() {
                    *shoot_timer = Timer::from_seconds(0.5, TimerMode::Once);
                }
            }
        }
    }
}

fn spawn_enemy_bullets(
    commands: &mut Commands,
    enemy_pos: Vec3,
    player_pos: Vec3,
    num_bullets: usize,
    handle: &Res<GlobalTextureAtlas>,
) {
    let direction = (player_pos - enemy_pos).normalize();
    for _ in 0..num_bullets {
        let spread = Vec3::new(
            rand::random::<f32>() * 0.2 - 0.1,
            rand::random::<f32>() * 0.2 - 0.1,
            0.0,
        );
        let bullet_direction = direction + spread;

        commands.spawn((
            SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_translation(enemy_pos)
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 81,
            },
            EnemyBullet,
            BulletDirection(bullet_direction),
            BulletStats {
                speed: 200,
                damage: 10,
                lifespan: 2.0,
            },
            InGameEntity,
            HasLifespan::new(Duration::from_secs(2)),
        ));
    }
}

pub fn update_enemy_bullets(
    mut bullet_query: Query<(&mut Transform, &BulletDirection, &BulletStats), With<EnemyBullet>>,
    time: Res<Time>,
) {
    for (mut transform, direction, stats) in bullet_query.iter_mut() {
        transform.translation += direction.0 * stats.speed as f32 * time.delta_seconds();
    }
}

pub fn handle_enemy_bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<EnemyBullet>>,
    player_query: Query<&Transform, (With<Player>, Without<InvincibilityEffect>)>,
    mut ev_player_damaged: EventWriter<PlayerDamagedEvent>,
) {
    if player_query.is_empty() {
        return;
    }
    let player_transform = player_query.single();

    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        if player_transform
            .translation
            .distance(bullet_transform.translation)
            < 30.0
        {
            ev_player_damaged.send(PlayerDamagedEvent { damage: 10 });
            println!("bullet collision!");
            // commands.entity(player_entity).insert(InvincibilityEffect(
            //     Stopwatch::new(),
            //     PLAYER_INVINCIBLE_TIME,
            // ));

            commands.entity(bullet_entity).despawn();
        }
    }
}

pub fn handle_bomber_death(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_query: Query<&Transform, (With<Player>, Without<InvincibilityEffect>)>,
    mut ev_player_damaged: EventWriter<PlayerDamagedEvent>,
    mut events: EventReader<BomberExplosionEvent>,
) {
    for event in events.read() {
        spawn_explosion(
            &mut commands,
            &mut meshes,
            &mut materials,
            event.translation,
            event.explosion_radius,
            event.explosion_damage,
        );

        if let Ok(player_transform) = player_query.get_single_mut() {
            let distance = player_transform.translation.distance(event.translation);
            if distance <= event.explosion_radius {
                ev_player_damaged.send(PlayerDamagedEvent {
                    damage: event.explosion_damage,
                });
                // commands.entity(player_entity).insert(InvincibilityEffect(
                //     Stopwatch::new(),
                //     PLAYER_INVINCIBLE_TIME,
                // ));
                println!("Explosion collision!");
            }
        }
    }
}

fn spawn_explosion(
    commands: &mut Commands,
    _meshes: &mut Assets<Mesh>,
    _materials: &mut Assets<ColorMaterial>,
    position: Vec3,
    radius: f32,
    damage: u32,
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(1.0, 0.5, 0.0, 0.5),
                custom_size: Some(Vec2::new(radius * 2.0, radius * 2.0)),
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        },
        Explosion {
            radius,
            damage,
            timer: Timer::from_seconds(0.3, TimerMode::Once),
        },
        InGameEntity,
    ));
}

pub fn update_explosions(
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
