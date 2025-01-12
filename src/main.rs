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
    enemy::EnemyPlugin,
    game_state::GameState,
    gun::GunPlugin,
    input::InputPlugin,
    player::{PlayerInventory, PlayerPlugin},
    potion::PotionPlugin,
    resources::ResourcesPlugin,
    ui::UiPlugin,
    world::WorldPlugin,
};
use iyes_perf_ui::PerfUiPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // mode: bevy::window::WindowMode::Fullscreen,
                        resizable: true,
                        canvas: Some("#fishmans_adventure".into()),
                        focused: true,
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
        .init_state::<GameState>()
        .run();
}

struct EmbeddedAssetPlugin;

impl Plugin for EmbeddedAssetPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "../assets/assets.png");
        embedded_asset!(app, "../assets/monogram.ttf");
        embedded_asset!(app, "../assets/icons/health.png");
        embedded_asset!(app, "../assets/icons/level.png");
        embedded_asset!(app, "../assets/icons/xp.png");
        embedded_asset!(app, "../assets/icons/defense.png");
    }
}
