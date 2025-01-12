use std::{f32::consts::PI, ops::RangeInclusive};

use bevy::{
    math::vec3,
    prelude::*,
    time::Stopwatch,
    utils::{Duration, Instant},
};
use leafwing_input_manager::prelude::ActionState;
use rand::Rng;

use crate::{
    collision::EnemyKdTree,
    configs::*,
    input::Action,
    loot::Description,
    player::{handle_player_input, Player, PlayerInventory},
    resources::GlobalTextureAtlas,
    state::GameState,
    utils::{get_nearest_enemy_position, InGameEntity},
};

use crate::enemy::{spawn_explosion, ExplodingBullet};

pub struct GunPlugin;

#[derive(Component)]
#[require(GunTimer, GunType, BulletStats(||BulletStats {
                speed: BULLET_SPEED,
                damage: BULLET_DAMAGE,
                lifespan: BULLET_TIME_SECS,
            }), GunStats(||GunStats {
                bullets_per_shot: NUM_BULLETS_PER_SHOT,
                firing_interval: BULLET_SPAWN_INTERVAL,
                bullet_spread: BULLET_SPREAD,
            }), InGameEntity, Sprite, Description)]
pub struct Gun;

#[derive(Component, Default)]
pub struct GunTimer(pub Stopwatch);

#[derive(Component, Clone, Default, Copy)]
pub enum GunType {
    #[default]
    SingleDirectionSpread,
    OmniSpread,
    FocusedAim,
}

#[derive(Component)]
pub struct ActiveGun;
#[derive(Component)]
pub struct Bullet;

#[derive(Component, Default)]
pub struct BulletStats {
    pub speed: u32,
    pub damage: u32,
    pub lifespan: f32,
}

#[derive(Component, Default)]
pub struct GunStats {
    pub bullets_per_shot: usize,
    pub firing_interval: f32,
    pub bullet_spread: f32,
}

#[derive(Component)]
pub struct HasLifespan {
    pub spawn_time: Instant,
    pub lifespan: Duration,
}

impl HasLifespan {
    pub fn new(lifespan: Duration) -> Self {
        HasLifespan {
            spawn_time: Instant::now(),
            lifespan,
        }
    }
}

#[derive(Component)]
pub struct BulletDirection(pub Vec3);

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_gun_transform.after(handle_player_input),
                move_bullets,
                handle_gun_firing,
                despawn_entities_reach_lifespan,
                switch_gun,
            )
                .run_if(in_state(GameState::Combat)),
        );
    }
}

fn update_gun_transform(
    player_query: Query<&Transform, With<Player>>,
    enemy_kd_tree: Res<EnemyKdTree>,
    mut gun_query: Query<&mut Transform, (With<ActiveGun>, Without<Player>)>,
) {
    let player_transform = if let Ok(transform) = player_query.get_single() {
        transform
    } else {
        return;
    };

    let mut gun_transform = if let Ok(transform) = gun_query.get_single_mut() {
        transform
    } else {
        return;
    };

    let player_pos = player_transform.translation.truncate();
    if let Some(nearest_enemy_pos) = get_nearest_enemy_position(player_pos, &enemy_kd_tree) {
        const MAX_RANGE: f32 = 500.0;
        if player_pos.distance(nearest_enemy_pos) <= MAX_RANGE {
            let angle =
                (player_pos.y - nearest_enemy_pos.y).atan2(player_pos.x - nearest_enemy_pos.x) + PI;
            gun_transform.rotation = Quat::from_rotation_z(angle);
        }
    }
    gun_transform.translation = vec3(
        player_pos.x + 10.0, // offset from player, need adjustment
        player_pos.y,
        gun_transform.translation.z,
    );
}

fn despawn_entities_reach_lifespan(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &HasLifespan, Option<&ExplodingBullet>)>,
) {
    for (entity, transform, lifespan, exploding) in bullet_query.iter() {
        if lifespan.spawn_time.elapsed() > lifespan.lifespan {
            if let Some(exploding_bullet) = exploding {
                spawn_explosion(
                    &mut commands,
                    transform.translation,
                    exploding_bullet.radius,
                    exploding_bullet.damage,
                );
            }
            commands.entity(entity).despawn();
        }
    }
}

fn handle_gun_firing(
    mut commands: Commands,
    time: Res<Time>,
    player_query: Query<(), With<Player>>,
    mut gun_query: Query<
        (&Transform, &mut GunTimer, &GunType, &BulletStats, &GunStats),
        With<ActiveGun>,
    >,
    handle: Res<GlobalTextureAtlas>,
) {
    if player_query.get_single().is_err() {
        return;
    }

    if let Ok((gun_transform, mut gun_timer, gun_type, bullet_stats, gun_stats)) =
        gun_query.get_single_mut()
    {
        gun_timer.0.tick(time.delta());

        if gun_timer.0.elapsed_secs() < gun_stats.firing_interval {
            return;
        }

        gun_timer.0.reset();
        let gun_pos = gun_transform.translation.truncate();
        let bullet_direction = gun_transform.local_x();

        match gun_type {
            GunType::SingleDirectionSpread => fire_bullets(
                &mut commands,
                gun_pos,
                *bullet_direction,
                gun_stats.bullets_per_shot,
                gun_stats.bullet_spread,
                bullet_stats,
                &handle,
                80..=83,
                *gun_type, // Texture index range for SingleDirectionSpread
            ),
            GunType::OmniSpread => fire_omni_bullets(
                &mut commands,
                gun_pos,
                gun_stats.bullets_per_shot,
                bullet_stats,
                &handle,
                84..=87,
                *gun_type, // Texture index range for OmniSpread
            ),
            GunType::FocusedAim => {
                // Specific logic for SingleDirection guns
                fire_bullets(
                    &mut commands,
                    gun_pos,
                    *bullet_direction,
                    1,   // Single bullet per shot
                    0.0, // No spread for SingleDirection
                    bullet_stats,
                    &handle,
                    84..=87,
                    *gun_type, // Texture index range for SingleDirection
                );
            }
        }
    }
}

/// A reusable function to fire bullets with the given parameters.
fn fire_bullets(
    commands: &mut Commands,
    gun_pos: Vec2,
    bullet_direction: Vec3,
    bullets_per_shot: usize,
    bullet_spread: f32,
    bullet_stats: &BulletStats,
    handle: &GlobalTextureAtlas,
    texture_index_range: RangeInclusive<usize>,
    gun_type: GunType,
) {
    let mut rng = rand::thread_rng();

    for _ in 0..bullets_per_shot {
        let dir = vec3(
            bullet_direction.x + rng.gen_range(-bullet_spread..=bullet_spread),
            bullet_direction.y + rng.gen_range(-bullet_spread..=bullet_spread),
            bullet_direction.z,
        );

        commands.spawn((
            Name::new("Bullet"),
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout_16x16.clone().unwrap(),
                    index: rng.gen_range(texture_index_range.clone()),
                }),
                ..default()
            },
            Transform::from_translation(vec3(gun_pos.x, gun_pos.y, LAYER3))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Bullet,
            BulletDirection(dir),
            BulletStats {
                speed: bullet_stats.speed,
                damage: bullet_stats.damage,
                lifespan: bullet_stats.lifespan,
            },
            gun_type,
            InGameEntity,
            HasLifespan::new(Duration::from_secs_f32(bullet_stats.lifespan)),
        ));
    }
}

/// Fires bullets in all directions (360-degree spread).
fn fire_omni_bullets(
    commands: &mut Commands,
    gun_pos: Vec2,
    bullets_per_shot: usize,
    bullet_stats: &BulletStats,
    handle: &GlobalTextureAtlas,
    texture_index_range: std::ops::RangeInclusive<usize>,
    gun_type: GunType,
) {
    let angle_step = 360.0 / bullets_per_shot as f32; // Divide the circle into equal parts
    let mut rng = rand::thread_rng();

    for i in 0..bullets_per_shot {
        let angle = i as f32 * angle_step; // Calculate the angle for this bullet
        let radians = angle.to_radians();
        let dir = vec3(radians.cos(), radians.sin(), 0.0); // Direction vector for the bullet

        commands.spawn((
            Name::new("Bullet"),
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout_16x16.clone().unwrap(),
                    index: rng.gen_range(texture_index_range.clone()),
                }),
                ..default()
            },
            Transform::from_translation(vec3(gun_pos.x, gun_pos.y, LAYER3))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Bullet,
            BulletDirection(dir),
            BulletStats {
                speed: bullet_stats.speed,
                damage: bullet_stats.damage,
                lifespan: bullet_stats.lifespan,
            },
            gun_type,
            InGameEntity,
            HasLifespan::new(Duration::from_secs_f32(bullet_stats.lifespan)),
        ));
    }
}

fn switch_gun(
    mut player_query: Query<(&mut PlayerInventory, &Transform), With<Player>>,
    action_state: Res<ActionState<Action>>,
    mut commands: Commands,
    mut gun_query: Query<(&mut Transform, &mut Visibility, Entity), (With<Gun>, Without<Player>)>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut inventory, player_transform) = player_query.single_mut();

    if action_state.just_pressed(&Action::SwitchGun) {
        inventory.active_gun_index = (inventory.active_gun_index + 1) % inventory.guns.len();
        for (gun_index, gun_entity) in inventory.guns.iter().enumerate() {
            if let Ok((mut gun_transform, mut gun_visibility, entity)) =
                gun_query.get_mut(*gun_entity)
            {
                if gun_index == inventory.active_gun_index {
                    gun_transform.translation = vec3(
                        player_transform.translation.x + 10.0, // offset from player, need adjustment
                        player_transform.translation.y,
                        gun_transform.translation.z,
                    );
                    commands.entity(entity).insert(ActiveGun);
                    *gun_visibility = Visibility::Visible;
                } else {
                    commands.entity(entity).remove::<ActiveGun>();
                    *gun_visibility = Visibility::Hidden;
                }
            }
        }
    }
}

fn move_bullets(
    mut bullet_query: Query<
        (&mut Transform, &mut BulletDirection, &BulletStats, &GunType),
        With<Bullet>,
    >,
    enemy_kd_tree: Res<EnemyKdTree>,
    player_query: Query<&Transform, (With<Player>, Without<Bullet>)>,
) {
    if bullet_query.is_empty() {
        return;
    }

    for (mut bullet_transform, mut bullet_direction, bullet_stats, gun_type) in
        bullet_query.iter_mut()
    {
        match gun_type {
            GunType::SingleDirectionSpread => {
                bullet_transform.translation +=
                    bullet_direction.0.normalize() * Vec3::splat(bullet_stats.speed as f32);
                bullet_transform.translation.z = LAYER4;
            }
            GunType::OmniSpread => {
                bullet_transform.translation +=
                    bullet_direction.0.normalize() * Vec3::splat(bullet_stats.speed as f32);
                bullet_transform.translation.z = LAYER4;
            }
            GunType::FocusedAim => {
                let player_transform = if let Ok(transform) = player_query.get_single() {
                    transform
                } else {
                    return;
                };
                let player_pos = player_transform.translation.truncate();
                if let Some(nearest_enemy_pos) =
                    get_nearest_enemy_position(player_pos, &enemy_kd_tree)
                {
                    const MAX_RANGE: f32 = 800.0;
                    let bullet_pos = bullet_transform.translation.truncate();
                    if bullet_pos.distance(nearest_enemy_pos) <= MAX_RANGE {
                        let new_direction = (nearest_enemy_pos - bullet_pos).normalize();
                        bullet_direction.0 = vec3(new_direction.x, new_direction.y, 0.0);
                    }
                    bullet_transform.translation +=
                        bullet_direction.0 * Vec3::splat(bullet_stats.speed as f32);
                    bullet_transform.translation.z = LAYER4;
                } else {
                    bullet_transform.translation +=
                        bullet_direction.0.normalize() * Vec3::splat(bullet_stats.speed as f32);
                    bullet_transform.translation.z = LAYER4;
                }
            }
        }
    }
}
