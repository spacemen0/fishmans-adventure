use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    Initializing,
    Combat,
    Paused,
    Ui,
    Shopping,
    End,
    Win,
}
