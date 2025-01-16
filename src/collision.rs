use bevy::utils::Duration;

use crate::{
    audio::AudioEvent,
    configs::KD_TREE_REFRESH_RATE,
    gun::BulletStats,
    player::{DamageBoost, InvincibilityEffect},
};
use bevy::{prelude::*, time::common_conditions::on_timer};
use kd_tree::{KdPoint, KdTree};

use crate::{
    enemy::{spawn_explosion, Enemy, ExplosionAbility, HitFlash, Trail},
    game_state::GameState,
    gun::Bullet,
    player::{Player, PlayerDamagedEvent},
};

pub struct CollisionPlugin;

#[derive(Component)]
pub struct Collidable {
    pub pos: Vec2,
    pub entity: Entity,
}

#[derive(Resource)]
pub struct EnemyKdTree(pub KdTree<Collidable>);

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

pub fn handle_enemy_player_collision(
    mut commands: Commands,
    player_query: Query<&Transform, (With<Player>, Without<InvincibilityEffect>)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy, Option<&ExplosionAbility>)>,
    tree: Res<EnemyKdTree>,
    mut ev: EventWriter<PlayerDamagedEvent>,
) {
    if player_query.is_empty() {
        return;
    }

    let player_transform = player_query.single();
    let player_pos = player_transform.translation;

    let enemies = tree.0.within_radius(&[player_pos.x, player_pos.y], 50.0);
    if enemies.is_empty() {
        return;
    }
    if let Ok((entity, transform, enemy_component, explosion_ability)) =
        enemy_query.get_mut(enemies.first().unwrap().entity)
    {
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
            return;
        }
        if enemy_component.damage > 0 {
            ev.send(PlayerDamagedEvent {
                damage: enemy_component.damage,
            });
            return;
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
            ev.send(PlayerDamagedEvent {
                damage: trail.damage,
            });
            return;
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
    bullet_query: Query<(&Transform, Entity, &BulletStats), With<Bullet>>,
    tree: Res<EnemyKdTree>,
    mut enemy_query: Query<(&Transform, &mut Enemy, Entity), With<Enemy>>,
    player_query: Query<&DamageBoost, With<Player>>,
    mut ew: EventWriter<AudioEvent>,
) {
    if bullet_query.is_empty() || enemy_query.is_empty() || player_query.is_empty() {
        return;
    }
    let player_damage_boost = player_query.get_single().unwrap().0;
    for (bullet_transform, bullet_entity, stats) in bullet_query.iter() {
        let pos = bullet_transform.translation;
        let enemies_in_radius = tree.0.within_radius(&[pos.x, pos.y], 30.0);

        if let Some(enemy) = enemies_in_radius.first() {
            if let Ok((_, mut enemy_component, enemy_entity)) = enemy_query.get_mut(enemy.entity) {
                if enemy_component.health > 0 {
                    enemy_component.health = enemy_component
                        .health
                        .saturating_sub(stats.damage + player_damage_boost);

                    if enemy_component.health > 0 {
                        ew.send(AudioEvent::Hit);
                        commands.entity(enemy_entity).insert(HitFlash::default());
                    }
                }

                commands.entity(bullet_entity).try_despawn();
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
