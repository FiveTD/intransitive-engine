use bevy::prelude::*;

pub const WINDOW_WIDTH: u32 = 1200;
pub const WINDOW_HEIGHT: u32 = 1200;

pub const BG_COLOR: Color = Color::WHITE;
pub const PLAYER_COLORS: [Color; 2] = [Color::hsl(0.0, 1.0, 0.5), Color::hsl(240.0, 1.0, 0.5)];
pub const CAPTURE_ALPHA: f32 = 0.5;

pub const BOARD_TILE_SIZE: f32 = 120.0;
pub const BOARD_LINE_THICKNESS: f32 = 3.0;
pub const BOARD_PIECE_SIZE: f32 = BOARD_TILE_SIZE * 0.8;
