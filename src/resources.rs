use std::collections::HashMap;

use bevy::prelude::*;

use crate::model::board::Board;

#[derive(Resource, Default)]
pub struct GameBoard(pub Board);

#[derive(Resource, Default)]
pub struct PieceEntities(pub HashMap<(usize, usize), Entity>);

#[derive(Resource, Default)]
pub struct SelectedTile(pub Option<(usize, usize)>);
