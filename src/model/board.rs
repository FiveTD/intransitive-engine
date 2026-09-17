use std::{
    cmp::min,
    ops::{Index, IndexMut},
};

// ============ PIECE DATA ============

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Rock,
    Paper,
    Scissors,
    Capture,
}

impl PieceType {
    pub fn resolve(a: PieceType, b: PieceType) -> Option<PieceType> {
        match (a, b) {
            (a, b) if a == b => None,
            (p, PieceType::Capture) | (PieceType::Capture, p) => Some(p),
            (PieceType::Rock, PieceType::Scissors)
            | (PieceType::Scissors, PieceType::Paper)
            | (PieceType::Paper, PieceType::Rock) => Some(a),
            _ => Some(b),
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

const BOARD_DEFAULT_SIZE: usize = 9;

#[derive(Debug)]
pub struct Board {
    size: usize,
    data: Vec<Option<Piece>>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            data: vec![None; size * size],
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn index(&self, x: usize, y: usize) -> usize {
        assert!(
            x < self.size && y < self.size,
            "coordinates out of bounds: ({x}, {y})",
        );
        y * self.size + x
    }

    pub fn tile(&self, x: usize, y: usize) -> &Option<Piece> {
        &self.data[self.index(x, y)]
    }

    pub fn tile_mut(&mut self, x: usize, y: usize) -> &mut Option<Piece> {
        let idx = self.index(x, y);
        &mut self.data[idx]
    }

    pub fn is_valid_move(&self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> bool {
        if !(0..self.size).contains(&from_x)
            || !(0..self.size).contains(&from_y)
            || !(from_x.saturating_sub(1)..min(from_x + 2, self.size)).contains(&to_x)
            || !(from_y.saturating_sub(1)..min(from_y + 2, self.size)).contains(&to_y)
        {
            false
        } else {
            if let Some(from) = self[from_x][from_y] {
                self[to_y][to_x].is_none_or(|to| {
                    from.owner != to.owner
                        && PieceType::resolve(from.piece_type, to.piece_type)
                            .is_some_and(|t| t == from.piece_type)
                })
            } else {
                false
            }
        }
    }

    pub fn moves_from(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = Vec::new();
        if self[y][x].is_none_or(|f| f.piece_type == PieceType::Capture) {
            return moves;
        }

        for iy in y.saturating_sub(1)..min(y + 2, self.size) {
            for ix in x.saturating_sub(1)..min(x + 2, self.size) {
                if self.is_valid_move(x, y, ix, iy) {
                    moves.push((ix, iy));
                }
            }
        }

        moves
    }

    pub fn default_setup(&mut self) {
        // assert!(self.size == BOARD_DEFAULT_SIZE, "board is not default size");

        // Corner capture squares
        self[0][BOARD_DEFAULT_SIZE - 1] = Some(Piece {
            piece_type: PieceType::Capture,
            owner: Player(0),
        });
        self[BOARD_DEFAULT_SIZE - 1][0] = Some(Piece {
            piece_type: PieceType::Capture,
            owner: Player(1),
        });

        // Initial piece diagonal setup
        for off in 1..BOARD_DEFAULT_SIZE - 4 {
            self[off][off + 3] = Some(Piece {
                piece_type: PieceType::Paper,
                owner: Player(0),
            });
            if off + 5 < BOARD_DEFAULT_SIZE {
                self[off + 1][off + 3] = Some(Piece {
                    piece_type: PieceType::Scissors,
                    owner: Player(0),
                });
                self[off][off + 4] = Some(Piece {
                    piece_type: PieceType::Rock,
                    owner: Player(0),
                });
            }

            // P1
            self[off + 3][off] = Some(Piece {
                piece_type: PieceType::Paper,
                owner: Player(1),
            });
            if off + 5 < BOARD_DEFAULT_SIZE {
                self[off + 3][off + 1] = Some(Piece {
                    piece_type: PieceType::Scissors,
                    owner: Player(1),
                });
                self[off + 4][off] = Some(Piece {
                    piece_type: PieceType::Rock,
                    owner: Player(1),
                });
            }
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new(BOARD_DEFAULT_SIZE)
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
