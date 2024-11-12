pub mod bundles;
pub mod components;
pub mod systems;
pub mod types;

use crate::state::GameState;
use bevy::prelude::*;

pub use self::{bundles::*, components::*, systems::*, types::*};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_enemies,
                update_enemy_behavior,
                despawn_dead_enemies,
                handle_enemy_collision,
                handle_shooter_enemies,
                update_enemy_bullets,
                handle_enemy_bullet_collision,
            )
                .run_if(in_state(GameState::Combat)),
        );
    }
}
