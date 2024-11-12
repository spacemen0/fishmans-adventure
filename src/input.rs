use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::KeyboardVirtualDPad;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    #[actionlike(DualAxis)]
    Move,
    SwitchGun,
    TogglePause,
    Restart,
    UsePotion1,
    UsePotion2,
}

impl Action {
    fn input_map() -> InputMap<Self> {
        InputMap::new([
            (Self::SwitchGun, KeyCode::KeyQ),
            (Self::TogglePause, KeyCode::KeyP),
            (Self::Restart, KeyCode::KeyR),
            (Self::UsePotion1, KeyCode::Digit1),
            (Self::UsePotion2, KeyCode::Digit2),
        ])
        .with_dual_axis(
            Self::Move,
            KeyboardVirtualDPad::new(KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD),
        )
    }
}
pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .init_resource::<ActionState<Action>>()
            .insert_resource(Action::input_map());
    }
}
