use crate::{
    game_state::GameState,
    input::Action,
    loot::Description,
    player::{Player, PlayerInventory},
    utils::InGameEntity,
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

#[derive(Component)]
#[require(ArmorStats, Sprite, InGameEntity, Description)]
pub struct Armor;

#[derive(Component, Default)]
pub struct ArmorStats {
    pub defense: u32,
    pub durability: u32,
}

#[derive(Component)]
pub struct ActiveArmor;
pub struct ArmorPlugin;

impl Plugin for ArmorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, switch_armor.run_if(in_state(GameState::Combat)));
    }
}

fn switch_armor(
    mut player_query: Query<&mut PlayerInventory, With<Player>>,
    active_state: Res<ActionState<Action>>,
    mut commands: Commands,
    armor_query: Query<Entity, With<Armor>>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut inventory = player_query.single_mut();

    if active_state.just_pressed(&Action::SwitchArmor) {
        inventory.active_armor_index = (inventory.active_armor_index + 1) % inventory.armors.len();
        for (index, entity) in armor_query.iter().enumerate() {
            if index == inventory.active_armor_index {
                commands.entity(entity).insert(ActiveArmor);
            } else {
                commands.entity(entity).remove::<ActiveArmor>();
            }
        }
    }
}
