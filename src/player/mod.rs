pub mod components;
pub mod systems;
use crate::game_state::GameState;
use bevy::prelude::*;
pub use components::*;
pub use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDamagedEvent>()
            .add_event::<PlayerLevelingUpEvent>()
            .add_systems(
                Update,
                (
                    handle_player_death,
                    handle_player_input,
                    handle_player_damaged_events,
                    handle_invincibility_effect,
                    handle_acceleration_effect,
                    handle_leveling_up,
                    handle_sprite_reset.run_if(any_component_removed::<InvincibilityEffect>),
                    handle_loot_pickup,
                    move_loot_to_player,
                    mark_loot_for_pickup,
                    update_player_invincibility_visual,
                )
                    .run_if(in_state(GameState::Combat)),
            );
    }
}
