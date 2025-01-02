use std::f32::consts::PI;

use bevy::{
    math::{vec2, vec3},
    prelude::*,
    time::Stopwatch,
    utils::{Duration, Instant},
};
use leafwing_input_manager::prelude::ActionState;
use rand::Rng;

use crate::{
    configs::*,
    input::Action,
    player::{handle_player_input, Player, PlayerInventory},
    resources::{CursorPosition, GlobalTextureAtlas},
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
    Gun2,
}

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
                .run_if(in_state(GameState::Combat).or(in_state(GameState::Town))),
        );
    }
}

fn update_gun_transform(
    cursor_pos: Res<CursorPosition>,
    player_query: Query<(&Transform, &PlayerInventory), With<Player>>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Player>)>,
) {
    if player_query.is_empty() || gun_query.is_empty() {
        return;
    }

    let (player_transform, gun_inventory) = player_query.single();
    let player_pos = player_transform.translation.truncate();

    // Get cursor position or default to player's position
    let cursor_pos = match cursor_pos.0 {
        Some(pos) => pos,
        None => player_pos,
    };

    // Retrieve the active gun from the player's inventory
    if let Some(active_gun_entity) = gun_inventory.guns.get(gun_inventory.active_gun_index) {
        if let Ok(mut gun_transform) = gun_query.get_mut(*active_gun_entity) {
            let angle = (player_pos.y - cursor_pos.y).atan2(player_pos.x - cursor_pos.x) + PI;
            gun_transform.rotation = Quat::from_rotation_z(angle);

            let offset = 20.0;
            let new_gun_pos = vec2(
                player_pos.x + offset * angle.cos() - 5.0,
                player_pos.y + offset * angle.sin() - 10.0,
            );

            gun_transform.translation =
                vec3(new_gun_pos.x, new_gun_pos.y, gun_transform.translation.z);
            gun_transform.translation.z = LAYER2;
        }
    }
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
    player_query: Query<&PlayerInventory, With<Player>>,
    mut gun_query: Query<(&Transform, &mut GunTimer, &GunType, &BulletStats, &GunStats), With<Gun>>,
    handle: Res<GlobalTextureAtlas>,
) {
    if let Ok(inventory) = player_query.get_single() {
        if let Ok((gun_transform, mut gun_timer, gun_type, bullet_stats, gun_stats)) =
            gun_query.get_mut(inventory.guns[inventory.active_gun_index])
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
                                        layout: handle.layout.clone().unwrap(),
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
                                        layout: handle.layout.clone().unwrap(),
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
                    GunType::Gun2 => todo!(),
                }
            }
        }
    }
}

fn switch_gun(
    mut player_query: Query<(&mut PlayerInventory, &Transform), With<Player>>,
    action_state: Res<ActionState<Action>>,
    mut gun_query: Query<(&mut Transform, &mut Visibility), (With<Gun>, Without<Player>)>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut inventory, player_transform) = player_query.single_mut();

    if action_state.just_pressed(&Action::SwitchGun) {
        // Cycle to the next gun in the inventory
        inventory.active_gun_index = (inventory.active_gun_index + 1) % inventory.guns.len();
    }

    // Update the visibility of all guns and the position of the active gun
    for (gun_index, gun_entity) in inventory.guns.iter().enumerate() {
        if let Ok((mut gun_transform, mut gun_visibility)) = gun_query.get_mut(*gun_entity) {
            if gun_index == inventory.active_gun_index {
                // Active gun
                gun_transform.translation = player_transform.translation;
                *gun_visibility = Visibility::Visible;
            } else {
                // Inactive gun
                *gun_visibility = Visibility::Hidden;
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
            GunType::Gun2 => todo!(),
        }
    }
}
