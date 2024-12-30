use bevy::utils::Duration;

use crate::{configs::KD_TREE_REFRESH_RATE, player::InvincibilityEffect, utils::safe_subtract};
use bevy::{prelude::*, time::common_conditions::on_timer};
use kd_tree::{KdPoint, KdTree};

use crate::{
    enemy::{Enemy, Trail, ExplosionAbility, spawn_explosion},
    gun::Bullet,
    player::{Player, PlayerDamagedEvent},
    state::GameState,
};

pub struct CollisionPlugin;

#[derive(Component)]
struct Collidable {
    pos: Vec2,
    entity: Entity,
}

#[derive(Resource)]
struct EnemyKdTree(KdTree<Collidable>);

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyKdTree::default()).add_systems(
            Update,
            (
                handle_enemy_bullet_collision,
                handle_enemy_player_collision,
                handle_player_trail_collision,
                update_enemy_kd_tree
                    .run_if(on_timer(Duration::from_secs_f32(KD_TREE_REFRESH_RATE))),
            )
                .run_if(in_state(GameState::Combat)),
        );
    }
}

fn handle_enemy_player_collision(
    mut commands: Commands,
    player_query: Query<&Transform, (With<Player>, Without<InvincibilityEffect>)>,
    mut enemy_query: Query<(
        Entity,
        &Transform,
        &mut Enemy,
        Option<&ExplosionAbility>
    )>,
    tree: Res<EnemyKdTree>,
    mut ev: EventWriter<PlayerDamagedEvent>,
) {
    if player_query.is_empty() {
        return;
    }

    let player_transform = player_query.single();
    let player_pos = player_transform.translation;

    let enemies = tree.0.within_radius(&[player_pos.x, player_pos.y], 50.0);
    for enemy in enemies {
        if let Ok((entity, transform, enemy_component, explosion_ability)) = 
            enemy_query.get_mut(enemy.entity) 
        {
            if enemy_component.damage > 0 {
                ev.send(PlayerDamagedEvent {
                    damage: enemy_component.damage,
                });
            }

            if let Some(explosion) = explosion_ability {
                spawn_explosion(
                    &mut commands,
                    transform.translation,
                    explosion.explosion_radius,
                    explosion.explosion_damage,
                );
                
                ev.send(PlayerDamagedEvent {
                    damage: explosion.explosion_damage,
                });
                
                commands.entity(entity).despawn();
            }
        }
    }
}

fn handle_player_trail_collision(
    mut player_query: Query<&Transform, (With<Player>, Without<InvincibilityEffect>)>,
    trail_query: Query<(&Transform, &Trail)>,
    mut ev: EventWriter<PlayerDamagedEvent>,
) {
    if player_query.is_empty() {
        return;
    }

    let translation = player_query.single_mut();
    let player_pos = translation.translation.xy();
    for (trail_transform, trail) in trail_query.iter() {
        let trail_pos = trail_transform.translation.xy();
        if player_pos.distance(trail_pos) <= trail.radius {
            println!("send trail event");
            ev.send(PlayerDamagedEvent {
                damage: trail.damage,
            });
            break;
        }
    }
}

fn update_enemy_kd_tree(
    mut tree: ResMut<EnemyKdTree>,
    enemy_query: Query<(&Transform, Entity), With<Enemy>>,
) {
    let mut items = Vec::new();
    for (t, e) in enemy_query.iter() {
        items.push(Collidable {
            entity: e,
            pos: t.translation.truncate(),
        })
    }

    tree.0 = KdTree::build_by_ordered_float(items);
}

fn handle_enemy_bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(&Transform, Entity), With<Bullet>>,
    tree: Res<EnemyKdTree>,
    mut enemy_query: Query<(&Transform, &mut Enemy), With<Enemy>>,
) {
    if bullet_query.is_empty() || enemy_query.is_empty() {
        return;
    }
    for (bullet_transform, bullet_entity) in bullet_query.iter() {
        let pos = bullet_transform.translation;
        let enemies_in_radius = tree.0.within_radius(&[pos.x, pos.y], 30.0);

        if let Some(enemy) = enemies_in_radius.first() {
            if let Ok((_, mut enemy)) = enemy_query.get_mut(enemy.entity) {
                enemy.health = safe_subtract(enemy.health, 55); //add bullet damage to gun
                commands.add(move |world: &mut World| {
                    if let Some(entity) = world.get_entity_mut(bullet_entity) {
                        entity.despawn();
                    }
                });
            }
        }
    }
}

impl KdPoint for Collidable {
    type Scalar = f32;
    type Dim = typenum::U2;
    fn at(&self, k: usize) -> f32 {
        if k == 0 {
            return self.pos.x;
        }
        self.pos.y
    }
}

impl Default for EnemyKdTree {
    fn default() -> Self {
        Self(KdTree::build_by_ordered_float(vec![]))
    }
}
