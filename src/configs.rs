// Window
pub const WW: f32 = 1080.0;
pub const WH: f32 = 720.0;

// Sprites
pub const SPRITE_SHEET_PATH: &str = "embedded://fishmans_adventure/../assets/assets.png";
pub const UI_FONT_PATH: &str = "embedded://fishmans_adventure/../assets/monogram.ttf";
pub const SPRITE_SCALE_FACTOR: f32 = 3.0;
pub const TILE_W: u32 = 16;
pub const TILE_H: u32 = 16;
pub const TILE_W_32: u32 = 32;
pub const TILE_H_32: u32 = 32;
pub const SPRITE_SHEET_W: u32 = 256 / TILE_W;
pub const SPRITE_SHEET_H: u32 = 256 / TILE_H;

// World
pub const NUM_WORLD_DECORATIONS: usize = 100;

// Player
pub const PLAYER_SPEED: u32 = 8;
pub const PLAYER_HEALTH: u32 = 50;
pub const PLAYER_INVINCIBLE_TIME: f32 = 0.1;
pub const MAX_DEFENSE: u32 = 30;

// Enemy
pub const REPEL_MARGIN: f32 = 100.0;

// Kd-tree
pub const KD_TREE_REFRESH_RATE: f32 = 0.1;

// Default Gun
pub const FIRING_INTERVAL: f32 = 0.2;
pub const BULLET_TIME_SECS: f32 = 0.5;
pub const BULLET_SPEED: u32 = 15;
pub const BULLET_DAMAGE: u32 = 25;
pub const BULLET_SPREAD: f32 = 0.7;
pub const NUM_BULLETS_PER_SHOT: usize = 5;

// Colors
pub const BG_COLOR: (u8, u8, u8) = (72, 59, 58);
pub const UI_BG_COLOR: (u8, u8, u8) = (197, 204, 184);
pub const LAYER0: f32 = 0.0;
pub const LAYER1: f32 = 1.0;
pub const LAYER2: f32 = 2.0;
pub const LAYER3: f32 = 3.0;
pub const LAYER4: f32 = 4.0;
pub const LAYER5: f32 = 5.0;
