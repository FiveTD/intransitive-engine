use bevy::prelude::*;

// use crate::model::board::*;
use crate::{events::MovePiece, resources::GameBoard};

pub fn setup_board(mut board: ResMut<GameBoard>) {
    board.0.default_setup();
}

pub fn handle_move_piece(move_piece: On<MovePiece>, mut board: ResMut<GameBoard>) {
    board.0.move_piece(move_piece.from, move_piece.to);
}
