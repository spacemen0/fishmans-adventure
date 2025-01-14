use crate::{
    game_state::GameState,
    player::{
        handle_acceleration_effect, handle_invincibility_effect, handle_leveling_up,
        handle_loot_pickup, handle_player_damaged_events, handle_player_death,
        handle_player_movement, handle_sprite_reset, mark_loot_for_pickup, move_loot_to_player,
        update_player_invincibility_visual, InvincibilityEffect, PlayerDamagedEvent,
        PlayerLevelingUpEvent,
    },
    ui::components::LootSaleEvent,
    utils::cleanup_entities,
};
use bevy::{
    app::{App, Plugin, Update},
    prelude::{any_component_removed, in_state, on_event, IntoSystemConfigs, OnEnter},
};

use super::handle_loot_sale_event;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDamagedEvent>()
            .add_event::<PlayerLevelingUpEvent>()
            .add_systems(
                Update,
                (
                    handle_player_death,
                    handle_player_movement,
                    handle_player_damaged_events.run_if(on_event::<PlayerDamagedEvent>),
                    handle_invincibility_effect,
                    handle_acceleration_effect,
                    handle_leveling_up.run_if(on_event::<PlayerLevelingUpEvent>),
                    handle_sprite_reset.run_if(any_component_removed::<InvincibilityEffect>),
                    handle_loot_pickup,
                    move_loot_to_player,
                    mark_loot_for_pickup,
                    update_player_invincibility_visual,
                )
                    .run_if(in_state(GameState::Combat)),
            )
            .add_systems(
                Update,
                handle_loot_sale_event.run_if(on_event::<LootSaleEvent>),
            )
            .add_systems(OnEnter(GameState::End), cleanup_entities);
    }
}
