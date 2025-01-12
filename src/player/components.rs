use bevy::{prelude::*, time::Stopwatch};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct OriginalColor(pub Color);

#[derive(Component, Reflect)]
pub struct Health(pub u32, pub u32);

//(current, max)
#[derive(Component, Reflect)]
pub struct Speed(pub u32);

#[derive(Component, Reflect)]
pub struct Defense(pub u32);

#[derive(Component, Reflect, Debug)]
pub struct PlayerInventory {
    pub guns: Vec<Entity>,
    pub health_potions: Vec<Entity>,
    pub speed_potions: Vec<Entity>,
    pub armors: Vec<Entity>,
    pub active_gun_index: usize,
    pub active_armor_index: usize,
}

#[derive(Component, Reflect)]
pub struct InvincibilityEffect(pub Stopwatch, pub f32);

#[derive(Component, Reflect)]
pub struct AccelerationEffect(pub Stopwatch, pub f32, pub u32);

#[derive(Component, Default, Debug, Reflect)]
pub enum PlayerState {
    #[default]
    Idle,
    Run,
}

#[derive(Event)]
pub struct PlayerDamagedEvent {
    pub damage: u32,
}

#[derive(Event)]
pub struct PlayerLevelingUpEvent {
    pub new_level: u32,
}
