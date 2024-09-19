use bevy::utils::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use kd_tree::{KdPoint, KdTree};
use player::{InvulnerableTimer, PlayerState};

use crate::player::{Player, PlayerEnemyCollisionEvent};
use crate::*;
use crate::{enemy::Enemy, gun::Bullet, state::GameState, enemy::Trail};


pub struct CollisionPlugin;

#[derive(Component)]
struct Collidable {
    pos: Vec2,
    entity: Entity,
    damage: f32,
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
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn handle_enemy_player_collision(
    mut player_query: Query<(&Transform, &mut PlayerState, &mut InvulnerableTimer), With<Player>>,
    tree: Res<EnemyKdTree>,
    mut ew: EventWriter<PlayerEnemyCollisionEvent>,
) {
    if player_query.is_empty() {
        return;
    }

    let (translation, mut player_state, mut invulnerable_timer) = player_query.single_mut();
    let player_pos = translation.translation;

    if matches!(*player_state, PlayerState::Idle | PlayerState::Run) {
        let enemies = tree.0.within_radius(&[player_pos.x, player_pos.y], 50.0);

        if !enemies.is_empty() {
            for enemy in enemies.iter() {
                if enemy.damage > 0.0 {
                    ew.send(PlayerEnemyCollisionEvent { damage: enemy.damage });
                }
            }
            if enemies.iter().any(|e| e.damage > 0.0) {
                match *player_state {
                    PlayerState::Idle => {
                        *player_state = PlayerState::IdleInvulnerable;
                        println!("{:?}", *player_state);
                    }
                    PlayerState::Run => {
                        *player_state = PlayerState::RunInvulnerable;
                        println!("{:?}", *player_state);
                    }
                    _ => unreachable!(),
                }
                invulnerable_timer.0.reset();
            }
        }
    }
}


fn handle_player_trail_collision(
    mut player_query: Query<(&Transform, &mut PlayerState, &mut InvulnerableTimer), With<Player>>,
    trail_query: Query<(&Transform, &Trail)>,
    mut ew: EventWriter<PlayerEnemyCollisionEvent>,
) {
    if player_query.is_empty() {
        return;
    }

    let (translation, mut player_state, mut invulnerable_timer) = player_query.single_mut();
    let player_pos = translation.translation.xy();

    if matches!(*player_state, PlayerState::Idle | PlayerState::Run) {
        for (trail_transform, trail) in trail_query.iter() {
            let trail_pos = trail_transform.translation.xy();
            if player_pos.distance(trail_pos) <= trail.radius {
                ew.send(PlayerEnemyCollisionEvent { damage: trail.damage });
                *player_state = match *player_state {
                    PlayerState::Idle => PlayerState::IdleInvulnerable,
                    PlayerState::Run => PlayerState::RunInvulnerable,
                    _ => unreachable!(),
                };
                invulnerable_timer.0.reset();
                break; 
            }
        }
    }
}




fn update_enemy_kd_tree(
    mut tree: ResMut<EnemyKdTree>,
    enemy_query: Query<(&Transform, Entity, &Enemy), With<Enemy>>,
) {
    let mut items = Vec::new();
    for (t, e, enemy) in enemy_query.iter() {
        items.push(Collidable {
            entity: e,
            pos: t.translation.truncate(),
            damage: enemy.damage,
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

    for b_t in bullet_query.iter() {
        let pos = b_t.0.translation;
        let enemies_in_radius = tree.0.within_radius(&[pos.x, pos.y], 30.0);

        if let Some(enemy) = enemies_in_radius.first() {
            if let Ok((_, mut enemy)) = enemy_query.get_mut(enemy.entity) {
                enemy.health -= BULLET_DAMAGE;
                commands.add(move |world: &mut World| {
                    if let Some(entity) = world.get_entity_mut(b_t.1) {
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
