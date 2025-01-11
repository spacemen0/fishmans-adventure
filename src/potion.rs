use crate::{
    input::Action,
    loot::Description,
    player::{AccelerationEffect, Health, Player, PlayerInventory, Speed},
    state::GameState,
    utils::InGameEntity,
};
use bevy::{prelude::*, time::Stopwatch};
use leafwing_input_manager::prelude::ActionState;

#[derive(Component)]
#[require(PotionStats, PotionType, InGameEntity, Sprite, Description)]
pub struct Potion;

#[derive(Component, Clone, Debug, Default)]
pub enum PotionType {
    #[default]
    Health,
    Speed,
}

#[derive(Component, Default)]
pub struct PotionStats {
    pub effect_duration: f32,
    pub effect_amount: u32,
}

pub struct PotionPlugin;

impl Plugin for PotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            apply_potion_effects.run_if(in_state(GameState::Combat)),
        );
    }
}

fn apply_potion_effects(
    mut commands: Commands,
    mut player_query: Query<(&mut Health, &mut PlayerInventory, Entity, &mut Speed), With<Player>>,
    potion_query: Query<(Entity, &PotionStats), With<Potion>>,
    action_state: Res<ActionState<Action>>,
) {
    let (mut health, mut player_inventory, entity, mut speed) = player_query.single_mut();
    if action_state.just_pressed(&Action::UsePotion1) {
        println!("Use potion 1");
        println!("Player inventory: {:?}", player_inventory);
        if let Some(health_potion_entity) = player_inventory.health_potions.first() {
            if let Ok((potion_entity, potion_stats)) = potion_query.get(*health_potion_entity) {
                health.0 = (health.0 + potion_stats.effect_amount).min(health.1);
                commands.entity(potion_entity).despawn();
                player_inventory.health_potions.remove(0);
            }
        }
    }

    if action_state.just_pressed(&Action::UsePotion2) {
        if let Some(speed_potion_entity) = player_inventory.speed_potions.first() {
            if let Ok((potion_entity, potion_stats)) = potion_query.get(*speed_potion_entity) {
                commands.entity(entity).insert(AccelerationEffect(
                    Stopwatch::new(),
                    potion_stats.effect_duration,
                    potion_stats.effect_amount,
                ));
                speed.0 += potion_stats.effect_amount;
                commands.entity(potion_entity).despawn();
                player_inventory.speed_potions.remove(0);
            }
        }
    }
}
