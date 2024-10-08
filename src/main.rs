use animation::AnimationPlugin;
use armor::ArmorPlugin;
use bevy::asset::embedded_asset;
use bevy::prelude::*;
use camera::FollowCameraPlugin;
use collision::CollisionPlugin;
use dialog::DialogPlugin;
use enemy::EnemyPlugin;
use fishmans_adventure::*;
use gui::GuiPlugin;
use gun::GunPlugin;
use player::PlayerPlugin;
use portal::PortalPlugin;
use potion::PotionPlugin;
use state::GameState;
use world::WorldPlugin;

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
        .add_plugins(EmbeddedAssetPlugin)
        .add_plugins(FollowCameraPlugin)
        .add_plugins(GuiPlugin)
        .add_plugins(GunPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(DialogPlugin)
        .add_plugins(PortalPlugin)
        .add_plugins(PotionPlugin)
        .add_plugins(ArmorPlugin)
        .add_systems(Update, exit_game)
        .init_state::<GameState>()
        .insert_resource(Msaa::Off)
        .run();
}

struct EmbeddedAssetPlugin;

impl Plugin for EmbeddedAssetPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "../assets/assets.png");
        embedded_asset!(app, "../assets/monogram.ttf");
    }
}

fn exit_game(mut exit: EventWriter<AppExit>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
