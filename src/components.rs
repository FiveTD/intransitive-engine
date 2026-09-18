use bevy::prelude::*;

// use crate::model::board::BoardPosition;

#[derive(Component)]
pub struct SelectionHighlight;

#[derive(Component)]
pub struct LegalMoveHighlight;

// #[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
// pub struct PiecePosition {
//     pub x: usize,
//     pub y: usize,
// }

// impl PiecePosition {
//     pub fn new(x: usize, y: usize) -> Self {
//         Self { x, y }
//     }
// }

// impl From<BoardPosition> for PiecePosition {
//     fn from((x, y): BoardPosition) -> Self {
//         Self { x, y }
//     }
// }

// impl From<PiecePosition> for BoardPosition {
//     fn from(value: PiecePosition) -> Self {
//         (value.x, value.y)
//     }
// }
