use crate::{
    input::Action,
    player::{Player, PlayerInventory},
    state::GameState,
    utils::InGameEntity,
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

#[derive(Component)]
pub struct Armor;

#[derive(Component)]
pub struct ArmorStats {
    pub defense: u32,
    pub durability: u32,
}

#[derive(Bundle)]
pub struct ArmorBundle {
    pub armor: Armor,
    pub armor_stats: ArmorStats,
    pub sprite_bundle: SpriteBundle,
    pub in_game_entity: InGameEntity,
    pub texture_bundle: TextureAtlas,
}

pub struct ArmorPlugin;

impl Plugin for ArmorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, switch_armor.run_if(in_state(GameState::Combat)));
    }
}

fn switch_armor(
    mut player_query: Query<&mut PlayerInventory, With<Player>>,
    active_state: Res<ActionState<Action>>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut inventory = player_query.single_mut();

    if active_state.just_pressed(&Action::SwitchArmor) {
        inventory.active_armor_index = (inventory.active_armor_index + 1) % inventory.armors.len();
    }
}
