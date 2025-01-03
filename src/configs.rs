// Window
pub const WW: f32 = 1080.0;
pub const WH: f32 = 672.0;

// Sprites
pub const SPRITE_SHEET_PATH: &str = "embedded://fishmans_adventure/../assets/assets.png";
pub const UI_FONT_PATH: &str = "embedded://fishmans_adventure/../assets/monogram.ttf";
pub const HEALTH_ICON_PATH: &str = "embedded://fishmans_adventure/../assets/icons/health.png";
pub const LEVEL_ICON_PATH: &str = "embedded://fishmans_adventure/../assets/icons/level.png";
pub const XP_ICON_PATH: &str = "embedded://fishmans_adventure/../assets/icons/xp.png";
pub const DEFENSE_ICON_PATH: &str = "embedded://fishmans_adventure/../assets/icons/defense.png";
pub const SPRITE_SCALE_FACTOR: f32 = 3.0;
pub const TILE_W: u32 = 16;
pub const TILE_H: u32 = 16;
pub const SPRITE_SHEET_W: u32 = 256 / TILE_W;
pub const SPRITE_SHEET_H: u32 = 256 / TILE_H;

// World
pub const NUM_WORLD_DECORATIONS: usize = 100;

// Player
pub const PLAYER_SPEED: u32 = 15;
pub const PLAYER_HEALTH: u32 = 100;
pub const PLAYER_INVINCIBLE_TIME: f32 = 1.0;

// Enemy
pub const MAX_NUM_ENEMIES: usize = 2000;
pub const ENEMY_DAMAGE: u32 = 1;
pub const ENEMY_HEALTH: u32 = 100;
pub const SPAWN_RATE_PER_SECOND: usize = 1000;
pub const ENEMY_SPAWN_INTERVAL: f32 = 1.0;
pub const ENEMY_SPEED: u32 = 6;

// Kd-tree
pub const KD_TREE_REFRESH_RATE: f32 = 0.1;

// Gun
pub const BULLET_SPAWN_INTERVAL: f32 = 0.2;
pub const BULLET_TIME_SECS: f32 = 0.5;
pub const BULLET_SPEED: u32 = 15;
pub const BULLET_DAMAGE: u32 = 55;
pub const BULLET_SPREAD: f32 = 0.7;
pub const NUM_BULLETS_PER_SHOT: usize = 5;

// Colors
pub const BG_COLOR: (u8, u8, u8) = (197, 204, 184);
pub const BORDER_COLOR: (u8, u8, u8) = (221, 115, 176);
pub const BORDER_THICKNESS: f32 = 10.0;
pub const LAYER0: f32 = 0.0;
pub const LAYER1: f32 = 1.0;
pub const LAYER2: f32 = 2.0;
pub const LAYER3: f32 = 3.0;
pub const LAYER4: f32 = 4.0;
