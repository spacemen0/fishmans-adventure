use super::*;
use crate::player::{Player, PlayerLevelingUpEvent};
use crate::resources::Wave;
use crate::utils::calculate_enemies_per_wave;
use crate::{GlobalTextureAtlas, Level};
use crate::{SPAWN_RATE_PER_SECOND, WORLD_H, WORLD_W};
use bevy::prelude::*;
use rand::Rng;

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
        let (x, y) = get_random_position_around(player_pos);
        let enemy_type = EnemyType::random();
        let _config = enemy_type.get_config();

        commands.spawn(EnemyBundle::new(enemy_type, Vec3::new(x, y, 1.0), &handle));

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
        let movement =
            enemy
                .behavior
                .update_movement(transform.translation, player_pos, speed, time.delta());
        transform.translation += movement;
        enemy
            .behavior
            .apply(&mut commands, &transform, time.delta());
    }
}

pub fn despawn_dead_enemies(
    mut commands: Commands,
    enemy_query: Query<(&Enemy, Entity), With<Enemy>>,
    mut wave: ResMut<Wave>,
    mut level: ResMut<Level>,
    mut ew: EventWriter<PlayerLevelingUpEvent>,
) {
    for (enemy, entity) in enemy_query.iter() {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn();
            wave.enemies_left -= 1;
            if level.add_xp(enemy.xp) {
                ew.send(PlayerLevelingUpEvent {
                    new_level: level.level(),
                });
            }
        }
    }

    if wave.enemies_left == 0 {
        if wave.number % 5 == 0 {
            wave.requires_portal = true;
        } else {
            wave.number += 1;
            let new_wave_count = calculate_enemies_per_wave(wave.number);
            wave.enemies_total = new_wave_count;
            wave.enemies_left = new_wave_count;
            wave.enemies_spawned = 0;
        }
    }
}

fn get_random_position_around(pos: Vec2) -> (f32, f32) {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let dist = rng.gen_range(1000.0..3000.0);

    let offset_x = angle.cos() * dist;
    let offset_y = angle.sin() * dist;

    let random_x = pos.x + offset_x;
    let random_y = pos.y + offset_y;

    (
        random_x.clamp(-WORLD_W, WORLD_W),
        random_y.clamp(-WORLD_H, WORLD_H),
    )
}

pub fn handle_enemy_collision(
    mut enemy_query: Query<(Entity, &mut Transform, &Collider), With<Enemy>>,
) {
    let mut combinations = enemy_query.iter_combinations_mut();
    while let Some(
        [(_entity_a, mut transform_a, collider_a), (_entity_b, mut transform_b, collider_b)],
    ) = combinations.fetch_next()
    {
        let distance = transform_a.translation.distance(transform_b.translation);
        let min_distance = collider_a.radius + collider_b.radius;

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

fn clamp_position(position: &mut Vec3) {
    position.x = position.x.clamp(-WORLD_W, WORLD_W);
    position.y = position.y.clamp(-WORLD_H, WORLD_H);
}
