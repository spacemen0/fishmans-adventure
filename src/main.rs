use bevy::{
    asset::embedded_asset, diagnostic::*, input::common_conditions::input_toggle_active, prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use fishmans_adventure::{
    animation::AnimationPlugin,
    armor::ArmorPlugin,
    camera::FollowCameraPlugin,
    collision::CollisionPlugin,
    configs::{BG_COLOR, WH, WW},
    enemy::plugin::EnemyPlugin,
    game_state::GameState,
    gun::GunPlugin,
    input::InputPlugin,
    player::{plugin::PlayerPlugin, PlayerInventory},
    potion::PotionPlugin,
    resources::ResourcesPlugin,
    ui::plugin::UiPlugin,
    world::WorldPlugin,
};
use iyes_perf_ui::PerfUiPlugin;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    fn notify_exit();
}

fn main() {
    let mut binding = App::new();
    let app = binding
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // mode: bevy::window::WindowMode::Fullscreen,
                        resizable: true,
                        canvas: Some("#fishmans_adventure".to_owned()),
                        title: "Fishman's Adventure".to_owned(),
                        focused: true,
                        // cursor_options: CursorOptions {
                        //     visible: false,
                        //     ..default()
                        // },
                        resolution: (WW, WH).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::srgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .register_type::<PlayerInventory>()
        .add_plugins(EmbeddedAssetPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(EntityCountDiagnosticsPlugin)
        .add_plugins(SystemInformationDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_plugins(FollowCameraPlugin)
        .add_plugins(GunPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(PotionPlugin)
        .add_plugins(ArmorPlugin)
        .add_plugins(InputPlugin)
        .init_state::<GameState>();
    #[cfg(target_arch = "wasm32")]
    app.add_systems(PostUpdate, handle_exit_event);
    app.run();
}

struct EmbeddedAssetPlugin;

impl Plugin for EmbeddedAssetPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "../assets/assets.png");
        embedded_asset!(app, "../assets/monogram.ttf");
    }
}

#[cfg(target_arch = "wasm32")]
fn handle_exit_event(mut exit_events: EventReader<AppExit>) {
    for _ in exit_events.read() {
        notify_exit();
    }
}
