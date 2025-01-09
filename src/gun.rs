use std::f32::consts::PI;

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
    player::{handle_player_input, Player, PlayerInventory},
    resources::GlobalTextureAtlas,
    state::GameState,
    utils::InGameEntity,
};

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
            }), InGameEntity, Sprite)]
pub struct Gun;

#[derive(Component, Default)]
pub struct GunTimer(pub Stopwatch);

#[derive(Component, Clone, Default)]
pub enum GunType {
    #[default]
    Default,
    Gun1,
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
                update_bullets,
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
    let nearest_enemy = enemy_kd_tree
        .0
        .nearest(&[player_pos.x, player_pos.y])
        .into_iter()
        .next();

    if let Some(nearest_enemy) = nearest_enemy {
        let nearest_enemy_pos = Vec2::new(nearest_enemy.item.pos[0], nearest_enemy.item.pos[1]);
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
    bullet_query: Query<(&HasLifespan, Entity)>,
) {
    for (lifespan, e) in bullet_query.iter() {
        if lifespan.spawn_time.elapsed() > lifespan.lifespan {
            commands.entity(e).despawn();
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
    if let Ok(_) = player_query.get_single() {
        if let Ok((gun_transform, mut gun_timer, gun_type, bullet_stats, gun_stats)) =
            gun_query.get_single_mut()
        {
            gun_timer.0.tick(time.delta());

            if gun_timer.0.elapsed_secs() >= gun_stats.firing_interval {
                gun_timer.0.reset();
                let gun_pos = gun_transform.translation.truncate();
                let mut rng = rand::thread_rng();
                let bullet_direction = gun_transform.local_x();
                match gun_type {
                    GunType::Default => {
                        for _ in 0..gun_stats.bullets_per_shot {
                            let dir = vec3(
                                bullet_direction.x
                                    + rng.gen_range(
                                        -gun_stats.bullet_spread..gun_stats.bullet_spread,
                                    ),
                                bullet_direction.y
                                    + rng.gen_range(
                                        -gun_stats.bullet_spread..gun_stats.bullet_spread,
                                    ),
                                bullet_direction.z,
                            );
                            commands.spawn((
                                Name::new("Bullet"),
                                Sprite {
                                    image: handle.image.clone().unwrap(),
                                    texture_atlas: Some(TextureAtlas {
                                        layout: handle.layout_16x16.clone().unwrap(),
                                        index: rng.gen_range(80..=83),
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
                                gun_type.clone(),
                                InGameEntity,
                                HasLifespan::new(Duration::from_secs_f32(bullet_stats.lifespan)),
                            ));
                        }
                    }
                    GunType::Gun1 => {
                        for _ in 0..gun_stats.bullets_per_shot {
                            let dir = vec3(
                                bullet_direction.x
                                    + rng.gen_range(
                                        -gun_stats.bullet_spread..gun_stats.bullet_spread,
                                    ),
                                bullet_direction.y
                                    + rng.gen_range(
                                        -gun_stats.bullet_spread..gun_stats.bullet_spread,
                                    ),
                                bullet_direction.z,
                            );
                            commands.spawn((
                                Name::new("Bullet"),
                                Sprite {
                                    image: handle.image.clone().unwrap(),
                                    texture_atlas: Some(TextureAtlas {
                                        layout: handle.layout_16x16.clone().unwrap(),
                                        index: rng.gen_range(84..=87),
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
                                gun_type.clone(),
                                InGameEntity,
                                HasLifespan::new(Duration::from_secs_f32(bullet_stats.lifespan)),
                            ));
                        }
                    }
                }
            }
        }
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

fn update_bullets(
    mut bullet_query: Query<
        (&mut Transform, &BulletDirection, &BulletStats, &GunType),
        With<Bullet>,
    >,
) {
    if bullet_query.is_empty() {
        return;
    }

    for (mut t, dir, stats, gun_type) in bullet_query.iter_mut() {
        match gun_type {
            GunType::Default => {
                t.translation += dir.0.normalize() * Vec3::splat(stats.speed as f32);
                t.translation.z = 10.0;
            }
            GunType::Gun1 => {
                t.translation += dir.0.normalize() * Vec3::splat(stats.speed as f32);
                t.translation.z = 10.0;
            }
        }
    }
}
