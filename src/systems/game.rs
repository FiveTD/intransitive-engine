use bevy::prelude::*;

// use crate::model::board::*;
use crate::resources::GameBoard;

pub fn setup_board(mut board: ResMut<GameBoard>) {
    board.0.default_setup();
}
