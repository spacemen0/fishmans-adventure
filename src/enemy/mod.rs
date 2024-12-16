pub mod bundles;
pub mod components;
pub mod systems;
pub mod types;

use crate::state::GameState;
use bevy::prelude::*;

pub use self::{bundles::*, components::*, systems::*, types::*};

#[derive(Event)]
pub struct BomberExplosionEvent {
    pub translation: Vec3,
    pub explosion_radius: f32,
    pub explosion_damage: u32,
}
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BomberExplosionEvent>().add_systems(
            Update,
            (
                spawn_enemies,
                update_enemy_behavior,
                despawn_dead_enemies,
                handle_enemy_collision,
                handle_shooter_enemies,
                update_enemy_bullets,
                handle_enemy_bullet_collision,
                handle_bomber_death,
                update_explosions,
            )
                .run_if(in_state(GameState::Combat)),
        );
    }
}
