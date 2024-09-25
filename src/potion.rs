use bevy::prelude::*;

use crate::{
    player::{Health, Player, PlayerInventory},
    state::GameState,
    world::InGameEntity,
};

#[derive(Component)]
pub struct Potion;

#[derive(Component)]
pub enum PotionType {
    Health,
    Speed,
}

#[derive(Component)]
pub struct PotionStats {
    pub effect_duration: f32,
    pub effect_amount: f32,
}

#[derive(Bundle)]
pub struct PotionBundle {
    pub potion: Potion,
    pub potion_stats: PotionStats,
    pub potion_type: PotionType,
    pub in_game_entity: InGameEntity,
    pub sprite_bundle: SpriteBundle,
}

pub struct PotionPlugin;

impl Plugin for PotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            apply_potion_effects.run_if(in_state(GameState::InGame)),
        );
    }
}

fn apply_potion_effects(
    mut commands: Commands,
    mut player_query: Query<(&mut Health, &mut PlayerInventory), With<Player>>,
    potion_query: Query<(Entity, &PotionStats), With<Potion>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // Ensure there's only one player entity
    let (mut health, mut player_inventory) = player_query.single_mut();
    // Assuming health and speed effects are keyed to specific keys
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        if let Some(health_potion_entity) = player_inventory.health_potions.get(0) {
            if let Ok((potion_entity, potion_stats)) = potion_query.get(*health_potion_entity) {
                // Apply health effect
                health.0 += potion_stats.effect_amount;
                commands.entity(potion_entity).despawn(); // Despawn the potion entity
                player_inventory.health_potions.remove(0);
            }
        }
    }

    if keyboard_input.just_pressed(KeyCode::Digit2) {
        if let Some(speed_potion_entity) = player_inventory.speed_potions.get(0) {
            if let Ok((potion_entity, potion_stats)) = potion_query.get(*speed_potion_entity) {
                // Apply speed potion effect
                println!("Applying Speed Potion: {}", potion_stats.effect_amount);
                commands.entity(potion_entity).despawn(); // Despawn the potion entity
                player_inventory.speed_potions.remove(0);
            }
        }
    }
}
