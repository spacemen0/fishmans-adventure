use super::{
    components::LootSaleEvent,
    systems::{
        in_game_ui::update_floating_text,
        loot_grid::highlight_active_item,
        menus::{
            despawn_floating_text_box, handle_control_widget, handle_shop_input,
            handle_shop_menu_buttons, setup_shop_menu,
        },
    },
};
use crate::ui::systems::player_info;
use crate::{
    game_state::GameState,
    ui::systems::{
        in_game_ui, loot_grid,
        menus::{self, handle_end_screen_input, set_up_death_screen, set_up_win_screen},
    },
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
        app.add_event::<LootSaleEvent>()
            .add_systems(
                OnEnter(GameState::Initializing),
                (
                    player_info::setup_ui,
                    menus::setup_pause_menu,
                    setup_shop_menu,
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
                    (menus::handle_main_menu_buttons, handle_control_widget)
                        .run_if(in_state(GameState::MainMenu)),
                    menus::blink_text,
                ),
            )
            .add_systems(
                Update,
                (player_info::toggle_loot_ui_visibility.run_if(
                    in_state(GameState::Combat)
                        .or(in_state(GameState::Ui))
                        .or(in_state(GameState::Paused)),
                ),),
            )
            .add_systems(OnEnter(GameState::Ui), loot_grid::set_up_loot_image)
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
                    loot_grid::navigate_loot_items,
                    loot_grid::highlight_focused_item,
                    loot_grid::handle_sell_focused_item,
                    player_info::update_ui,
                    highlight_active_item,
                )
                    .run_if(in_state(GameState::Ui)),
            )
            .add_systems(OnEnter(GameState::End), set_up_death_screen)
            .add_systems(
                Update,
                (
                    handle_end_screen_input
                        .run_if(in_state(GameState::End).or(in_state(GameState::Win))),
                    update_floating_text,
                ),
            )
            .add_systems(OnEnter(GameState::Win), set_up_win_screen)
            .add_systems(
                Update,
                handle_shop_input
                    .run_if(in_state(GameState::Combat).or(in_state(GameState::Shopping))),
            )
            .add_systems(
                Update,
                (
                    (handle_shop_menu_buttons).run_if(in_state(GameState::Shopping)),
                    despawn_floating_text_box,
                ),
            );
    }
}
