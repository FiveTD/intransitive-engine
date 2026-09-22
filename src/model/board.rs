use std::{
    cmp::min,
    collections::HashMap,
    ops::{Index, IndexMut},
};

use minimax::GameState;

use crate::constants::*;

// ============ PIECE DATA ============

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Rock,
    Paper,
    Scissors,
}

impl PieceType {
    pub fn resolve(a: PieceType, b: PieceType) -> Option<PieceType> {
        match (a, b) {
            (a, b) if a == b => None,
            (PieceType::Rock, PieceType::Scissors)
            | (PieceType::Scissors, PieceType::Paper)
            | (PieceType::Paper, PieceType::Rock) => Some(a),
            _ => Some(b),
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Player(pub u8);

impl From<u8> for Player {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<Player> for u8 {
    fn from(value: Player) -> Self {
        value.0
    }
}

impl From<Player> for usize {
    fn from(value: Player) -> Self {
        value.0 as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub owner: Player,
}

// ============ BOARD ============

pub type BoardPosition = (usize, usize);

#[derive(Debug, Clone)]
pub struct Board {
    size: usize,
    data: Vec<Option<Piece>>,
    num_players: u8,
    active_turn: Player,
    capture_tiles: HashMap<Player, BoardPosition>,
}

impl Board {
    pub fn new(size: usize, num_players: u8) -> Self {
        debug_assert!(num_players >= 2, "not enough players");

        Self {
            size,
            data: vec![None; size * size],
            num_players,
            active_turn: Player(0),
            capture_tiles: HashMap::with_capacity(num_players as usize),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn active_turn(&self) -> Player {
        self.active_turn
    }

    pub fn capture_tiles(&self) -> &HashMap<Player, BoardPosition> {
        &self.capture_tiles
    }

    fn index(&self, (x, y): BoardPosition) -> usize {
        assert!(
            x < self.size && y < self.size,
            "coordinates out of bounds: ({x}, {y})",
        );
        y * self.size + x
    }

    fn tile(&self, pos: BoardPosition) -> &Option<Piece> {
        &self.data[self.index(pos)]
    }

    fn tile_mut(&mut self, pos: BoardPosition) -> &mut Option<Piece> {
        let idx = self.index(pos);
        &mut self.data[idx]
    }

    fn next_player(&self) -> Player {
        Player(if self.active_turn.0 >= self.num_players - 1 {
            0
        } else {
            self.active_turn.0 + 1
        })
    }

    pub fn move_piece(&mut self, from: BoardPosition, to: BoardPosition) {
        debug_assert!(
            self.is_valid_move(from, to),
            "invalid move from ({}, {}) to ({}, {})",
            from.0,
            from.1,
            to.0,
            to.1
        );

        self[to] = self[from];
        self[from] = None;
        self.active_turn = self.next_player()
    }

    pub fn clone_with_move(&self, from: BoardPosition, to: BoardPosition) -> Self {
        let mut clone = self.clone();
        clone.move_piece(from, to);
        clone
    }

    pub fn is_valid_move(&self, (fx, fy): BoardPosition, (tx, ty): BoardPosition) -> bool {
        if !(0..self.size).contains(&fx)
            || !(0..self.size).contains(&fy)
            || !(fx.saturating_sub(1)..min(fx + 2, self.size)).contains(&tx)
            || !(fy.saturating_sub(1)..min(fy + 2, self.size)).contains(&ty)
        {
            false
        } else {
            if let Some(from) = self[fy][fx] {
                from.owner == self.active_turn
                    && self[ty][tx].is_none_or(|to| {
                        from.owner != to.owner
                            && PieceType::resolve(from.piece_type, to.piece_type)
                                .is_some_and(|t| t == from.piece_type)
                    })
            } else {
                false
            }
        }
    }

    pub fn moves_from(&self, pos: BoardPosition) -> Vec<BoardPosition> {
        let mut moves: Vec<BoardPosition> = Vec::new();
        if self.tile(pos).is_none_or(|f| f.owner != self.active_turn) {
            return moves;
        }

        let (x, y) = pos;
        for iy in y.saturating_sub(1)..min(y + 2, self.size) {
            for ix in x.saturating_sub(1)..min(x + 2, self.size) {
                if self.is_valid_move(pos, (ix, iy)) {
                    moves.push((ix, iy));
                }
            }
        }

        moves
    }

    pub fn winner(&self) -> Option<Player> {
        self.capture_tiles
            .iter()
            .find_map(|(&player, &pos)| {
                self[pos].and_then(|piece| (piece.owner != player).then_some(piece.owner))
            })
            .or_else(|| self.legal_moves().is_empty().then_some(self.next_player()))
    }

    pub fn default_setup(&mut self) {
        debug_assert!(
            self.size >= 5,
            "board is not large enough for default setup"
        );
        debug_assert!(self.num_players == 2, "board has non-default player count");
        let size = self.size;

        // Corner capture tiles
        self.capture_tiles.insert(Player(0), (0, size - 1));
        self.capture_tiles.insert(Player(1), (size - 1, 0));

        // Initial piece diagonal setup
        for off in 1..size - 4 {
            self[off][off + 3] = Some(Piece {
                piece_type: PieceType::Paper,
                owner: Player(1),
            });
            if off + 5 < size {
                self[off + 1][off + 3] = Some(Piece {
                    piece_type: PieceType::Scissors,
                    owner: Player(1),
                });
                self[off][off + 4] = Some(Piece {
                    piece_type: PieceType::Rock,
                    owner: Player(1),
                });
            }

            // P1
            self[off + 3][off] = Some(Piece {
                piece_type: PieceType::Paper,
                owner: Player(0),
            });
            if off + 5 < size {
                self[off + 3][off + 1] = Some(Piece {
                    piece_type: PieceType::Scissors,
                    owner: Player(0),
                });
                self[off + 4][off] = Some(Piece {
                    piece_type: PieceType::Rock,
                    owner: Player(0),
                });
            }
        }
    }
}

impl minimax::GameState for Board {
    type Move = (BoardPosition, BoardPosition);

    fn legal_moves(&self) -> Vec<Self::Move> {
        (0..self.size)
            .flat_map(|y| (0..self.size).map(move |x| (x, y)))
            .filter(|&pos| self[pos].is_some_and(|p| p.owner == self.active_turn))
            .flat_map(|from| self.moves_from(from).into_iter().map(move |to| (from, to)))
            .collect()
    }

    fn is_terminal(&self) -> bool {
        self.winner().is_some()
    }

    fn apply_move(&self, (from, to): &Self::Move) -> Self {
        self.clone_with_move(*from, *to)
    }

    fn evaluate(&self) -> i32 {
        todo!()
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new(BOARD_SIZE, MAX_PLAYERS)
    }
}

impl Index<usize> for Board {
    type Output = [Option<Piece>];

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size);
        let row_index = index * self.size;
        &self.data[row_index..row_index + self.size]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.size);
        let row_index = index * self.size;
        &mut self.data[row_index..row_index + self.size]
    }
}

impl Index<BoardPosition> for Board {
    type Output = Option<Piece>;

    fn index(&self, index: BoardPosition) -> &Self::Output {
        self.tile(index)
    }
}

impl IndexMut<BoardPosition> for Board {
    fn index_mut(&mut self, index: BoardPosition) -> &mut Self::Output {
        self.tile_mut(index)
    }
}
