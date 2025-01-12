use crate::{
    enemy::{
        handle_charge_abilities, handle_enemy_bullet_player_collision, handle_enemy_death,
        handle_enemy_splitting, handle_exploding_bullets, handle_explosion_player_collision,
        handle_explosions, handle_ranged_movement, handle_shooting_abilities,
        handle_summoning_abilities, handle_trail_abilities, spawn_enemies, update_enemy_bullets,
        update_enemy_movement, BomberExplosionEvent,
    },
    game_state::GameState,
};
use bevy::{
    app::{App, Plugin, Update},
    prelude::{in_state, IntoSystemConfigs},
};

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
