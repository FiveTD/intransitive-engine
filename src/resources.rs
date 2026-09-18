use std::collections::HashMap;

use bevy::prelude::*;

use crate::model::board::*;

#[derive(Resource, Default)]
pub struct GameBoard(pub Board);

#[derive(Resource, Default)]
pub struct PieceEntities(pub HashMap<BoardPosition, Entity>);

#[derive(Resource, Default)]
pub struct SelectedTile(pub Option<BoardPosition>);
