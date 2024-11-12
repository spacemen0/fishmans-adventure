use crate::{
    player::{Player, PlayerInventory},
    state::GameState,
    utils::InGameEntity,
};
use bevy::prelude::*;

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
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut inventory = player_query.single_mut();

    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        inventory.active_armor_index = (inventory.active_armor_index + 1) % inventory.armors.len();
    }
}
