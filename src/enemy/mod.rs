pub mod builder;
pub mod components;
pub mod presets;
pub mod systems;

pub use builder::*;
pub use components::*;
pub use presets::*;
pub use systems::*;

use crate::state::GameState;
use bevy::prelude::*;

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
                update_enemy_movement,
                update_enemy_bullets,
                handle_ranged_movement,
                handle_trail_abilities,
                handle_shooting_abilities,
                handle_charge_abilities,
                handle_enemy_death,
                handle_explosions,
                handle_enemy_bullet_player_collision,
                handle_exploding_bullets,
                handle_explosion_player_collision,
                handle_enemy_splitting,
                handle_summoning_abilities,
            )
                .run_if(in_state(GameState::Combat)),
        );
    }
}
