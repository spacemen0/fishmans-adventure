use bevy::prelude::*;

use crate::{configs::*, game_state::GameState};

pub struct ResourcesPlugin;

#[derive(Resource, Default)]
pub struct GlobalTextureAtlas {
    pub layout_16x16: Option<Handle<TextureAtlasLayout>>,
    pub layout_32x32: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}
#[derive(Resource)]
pub struct UiFont(pub Handle<Font>);

#[derive(Resource, Default)]
pub struct Wave {
    pub number: u32,
}

#[derive(Resource, Clone, Copy, PartialEq)]
pub enum GameMode {
    Normal,
    Forever,
}

impl Default for GameMode {
    fn default() -> Self {
        Self::Normal
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
            .add_systems(OnEnter(GameState::Loading), load_assets);
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

    let layout_16x16 = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_W, TILE_H),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );
    let layout_32x32 = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_W * 2, TILE_H * 2),
        SPRITE_SHEET_W / 2,
        SPRITE_SHEET_H / 2,
        None,
        None,
    );

    handle.layout_16x16 = Some(texture_atlas_layouts.add(layout_16x16));
    handle.layout_32x32 = Some(texture_atlas_layouts.add(layout_32x32));

    next_state.set(GameState::MainMenu);
}
