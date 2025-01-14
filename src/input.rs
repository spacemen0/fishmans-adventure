use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    #[actionlike(DualAxis)]
    Move,
    SwitchGun,
    TogglePause,
    SwitchArmor,
    Confirm,
    NavigateUp,
    NavigateDown,
    NavigationLeft,
    NavigationRight,
    UsePotion1,
    UsePotion2,
    ToggleLootBoard,
    SellLoot,
    ToggleShop,
}

impl Action {
    fn input_map() -> InputMap<Self> {
        InputMap::new([
            (Self::SwitchGun, KeyCode::KeyQ),
            (Self::TogglePause, KeyCode::KeyP),
            (Self::SwitchArmor, KeyCode::KeyZ),
            (Self::Confirm, KeyCode::Enter),
            (Self::NavigateUp, KeyCode::ArrowUp),
            (Self::NavigateUp, KeyCode::KeyW),
            (Self::NavigateDown, KeyCode::ArrowDown),
            (Self::NavigateDown, KeyCode::KeyS),
            (Self::NavigationLeft, KeyCode::ArrowLeft),
            (Self::NavigationLeft, KeyCode::KeyA),
            (Self::NavigationRight, KeyCode::ArrowRight),
            (Self::NavigationRight, KeyCode::KeyD),
            (Self::UsePotion1, KeyCode::Digit1),
            (Self::UsePotion2, KeyCode::Digit2),
            (Self::ToggleLootBoard, KeyCode::Tab),
            (Self::SellLoot, KeyCode::Delete),
            (Self::ToggleShop, KeyCode::KeyO),
        ])
        .with_dual_axis(
            Self::Move,
            VirtualDPad::new(KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD),
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
