use bevy::prelude::*;

pub const WINDOW_WIDTH: u32 = 1200;
pub const WINDOW_HEIGHT: u32 = 1200;

pub const BG_COLOR: Color = Color::WHITE;
pub const MAX_PLAYERS: u8 = 2;
pub const PLAYER_COLORS: [Color; MAX_PLAYERS as usize] =
    [Color::hsl(0.0, 1.0, 0.5), Color::hsl(240.0, 1.0, 0.5)];
pub const CAPTURE_ALPHA: f32 = 0.5;
pub const SELECTION_COLOR: Color = Color::hsla(300., 1.0, 0.5, 0.25);
pub const LEGAL_MOVE_COLOR: Color = Color::hsla(108., 1.0, 0.2, 0.25);

pub const BOARD_SIZE: usize = 9;
pub const BOARD_TILE_SIZE: f32 = 120.0;
pub const BOARD_LINE_THICKNESS: f32 = 3.0;
pub const BOARD_PIECE_SIZE: f32 = BOARD_TILE_SIZE * 0.8;
