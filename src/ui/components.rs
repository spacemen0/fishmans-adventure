use bevy::prelude::*;

#[derive(Component)]
pub struct PauseMenuRoot;

#[derive(Component)]
pub struct DescriptionTextBox;

#[derive(Component)]
pub enum MenuButton {
    Resume,
    Restart,
    Quit,
}

#[derive(Component)]
pub struct MainMenuRoot;

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
pub struct PlayerHealthBar;

#[derive(Component)]
pub struct UiRoot;

#[derive(Component)]
pub struct WaveDisplayRoot;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridSlot {
    pub x: usize,
    pub y: usize,
    pub item: Option<Entity>,
}

#[derive(Component)]
pub struct FocusedItem;
