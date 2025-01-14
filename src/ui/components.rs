use bevy::prelude::*;

use crate::loot::LootType;

#[derive(Component)]
pub struct PauseMenuRoot;

#[derive(Component)]
pub struct PauseMenuButtonIndex(pub u8);

#[derive(Component)]
pub struct DescriptionTextBox;

#[derive(Event)]
pub struct LootSaleEvent(pub Entity, pub LootType);

#[derive(Component)]
pub struct FloatingText;

#[derive(Component)]
pub enum PauseMenuButton {
    Resume,
    Restart,
    Quit,
}

#[derive(Component)]
pub struct MainMenuRoot;

#[derive(Component)]
pub enum MainMenuButton {
    Start,
    Control,
    Exit,
}

#[derive(Component)]
pub struct MainMenuButtonIndex(pub u8);
#[derive(Component)]
pub struct ControlWidget;

#[derive(Component)]
pub struct BlinkingText;

#[derive(Component)]
pub struct DeathScreenRoot;

#[derive(Component)]
pub struct PlayerHealthText;

#[derive(Component)]
pub struct PlayerLevelText;

#[derive(Component)]
pub struct WaveDisplay;

#[derive(Component)]
pub struct PlayerXpText;

#[derive(Component)]
pub struct PlayerDefenseText;

#[derive(Component)]
pub struct PlayerGoldText;

#[derive(Component)]
pub struct PlayerHealthBar;

#[derive(Component)]
pub struct PlayerDamageBoostText;

#[derive(Component)]
pub struct UiRoot;

#[derive(Component)]
pub struct WaveDisplayRoot;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub struct GridSlot {
    pub x: usize,
    pub y: usize,
    pub item: Option<Entity>,
}

#[derive(Component)]
pub struct FocusedItem;
