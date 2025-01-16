use crate::{
    enemy::{
        handle_charge_abilities, handle_charge_enemy_flash, handle_death_effect,
        handle_enemy_bullet_player_collision, handle_enemy_death, handle_enemy_splitting,
        handle_exploding_bullets, handle_explosion_player_collision, handle_explosions,
        handle_hit_flash, handle_ranged_movement, handle_shooting_abilities,
        handle_summoning_abilities, handle_trail_abilities, spawn_enemies, update_enemy_bullets,
        update_enemy_movement, update_spawn_indicators, BomberExplosionEvent,
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
                update_spawn_indicators,
                (update_enemy_movement, handle_ranged_movement).chain(),
                (
                    handle_trail_abilities,
                    handle_shooting_abilities,
                    handle_charge_abilities,
                    handle_summoning_abilities,
                )
                    .after(update_enemy_movement),
                (
                    update_enemy_bullets,
                    handle_enemy_bullet_player_collision,
                    handle_exploding_bullets,
                )
                    .after(handle_shooting_abilities),
                (handle_explosions, handle_explosion_player_collision)
                    .after(handle_exploding_bullets),
                (
                    handle_charge_enemy_flash,
                    handle_hit_flash,
                    handle_death_effect,
                ),
                (handle_enemy_death, handle_enemy_splitting)
                    .after(handle_enemy_bullet_player_collision),
            )
                .run_if(in_state(GameState::Combat)),
        );
    }
}
