use bevy::prelude::*;

use crate::model::board::BoardPosition;

#[derive(Event)]
pub struct MovePiece {
    pub from: BoardPosition,
    pub to: BoardPosition,
}
