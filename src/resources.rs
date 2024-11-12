use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::state::GameState;
use crate::*;

pub struct ResourcesPlugin;

#[derive(Resource, Default)]
pub struct GlobalTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}
#[derive(Resource)]
pub struct CursorPosition(pub Option<Vec2>);
#[derive(Resource)]
pub struct UiFont(pub Handle<Font>);

#[derive(Resource)]
pub struct Wave {
    pub number: u32,
    pub enemies_left: u32,
    pub enemies_total: u32,
    pub requires_portal: bool,
    pub enemies_spawned: u32,
    pub portal_spawned: bool,
}

impl Default for Wave {
    fn default() -> Self {
        Self {
            number: 1,
            enemies_left: 1,
            enemies_total: 1,
            requires_portal: false,
            enemies_spawned: 0,
            portal_spawned: false,
        }
    }
}

#[derive(Resource)]
pub struct Level {
    current_xp: u32,
    xp_threshold: u32,
    level: u32,
}

impl Default for Level {
    fn default() -> Self {
        Self {
            current_xp: 0,
            xp_threshold: 100,
            level: 1,
        }
    }
}

impl Level {
    pub fn add_xp(&mut self, xp: u32) -> bool {
        self.current_xp += xp;
        let mut leveled_up = false;
        while self.current_xp >= self.xp_threshold {
            self.level_up();
            leveled_up = true;
        }
        leveled_up
    }

    fn level_up(&mut self) {
        self.current_xp -= self.xp_threshold;
        self.level += 1;
        self.xp_threshold = self.xp_threshold + self.xp_threshold / 2;
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn current_xp(&self) -> u32 {
        self.current_xp
    }

    pub fn xp_threshold(&self) -> u32 {
        self.xp_threshold
    }
}

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalTextureAtlas::default())
            .insert_resource(CursorPosition(None))
            .add_systems(OnEnter(GameState::Loading), load_assets)
            .add_systems(
                Update,
                update_cursor_position.run_if(in_state(GameState::Combat)),
            );
    }
}

fn load_assets(
    mut commands: Commands,
    mut handle: ResMut<GlobalTextureAtlas>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    handle.image = Some(asset_server.load(SPRITE_SHEET_PATH));
    commands.insert_resource(UiFont(asset_server.load(UI_FONT_PATH)));
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_W, TILE_H),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );
    handle.layout = Some(texture_atlas_layouts.add(layout));

    next_state.set(GameState::MainMenu);
}

fn update_cursor_position(
    mut cursor_pos: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    if window_query.is_empty() || camera_query.is_empty() {
        cursor_pos.0 = None;
        return;
    }

    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();
    cursor_pos.0 = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate());
}
