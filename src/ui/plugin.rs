use crate::ui::systems::menus::{handle_death_screen_input, set_up_death_screen};
use crate::{
    game_state::GameState,
    ui::systems::{in_game_ui, loot_ui, menus},
    utils::cleanup_entities,
    world::init_world,
};
use bevy::{
    app::{App, Plugin, Update},
    prelude::{in_state, Condition, IntoSystemConfigs, OnEnter, OnExit},
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Initializing),
            (
                loot_ui::setup_ui,
                menus::setup_pause_menu,
                in_game_ui::setup_health_bar.after(init_world),
            ),
        )
        .add_systems(
            Update,
            menus::pause_menu_navigation.run_if(in_state(GameState::Paused)),
        )
        .add_systems(
            OnEnter(GameState::MainMenu),
            (menus::setup_main_menu, cleanup_entities),
        )
        .add_systems(OnExit(GameState::MainMenu), menus::despawn_main_menu)
        .add_systems(OnEnter(GameState::Combat), in_game_ui::setup_wave_display)
        .add_systems(
            Update,
            (
                menus::handle_main_menu_buttons.run_if(in_state(GameState::MainMenu)),
                menus::blink_text,
            ),
        )
        .add_systems(
            Update,
            (loot_ui::toggle_loot_ui_visibility.run_if(
                in_state(GameState::Combat)
                    .or(in_state(GameState::Ui))
                    .or(in_state(GameState::Paused)),
            ),),
        )
        .add_systems(
            OnEnter(GameState::Ui),
            (loot_ui::update_ui, loot_ui::set_up_loot_image),
        )
        .add_systems(
            Update,
            in_game_ui::update_wave_display
                .run_if(in_state(GameState::Combat).or(in_state(GameState::Paused))),
        )
        .add_systems(
            Update,
            (menus::handle_pause_input, in_game_ui::update_health_bar)
                .run_if(in_state(GameState::Combat).or(in_state(GameState::Paused))),
        )
        .add_systems(
            Update,
            (
                loot_ui::navigate_loot_items,
                loot_ui::highlight_focused_item,
            )
                .run_if(in_state(GameState::Ui)),
        )
        .add_systems(OnEnter(GameState::End), set_up_death_screen)
        .add_systems(
            Update,
            handle_death_screen_input.run_if(in_state(GameState::End)),
        );
    }
}
