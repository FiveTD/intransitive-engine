use std::{
    cmp::{max, min},
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

    pub fn moves_from(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = Vec::new();
        let from = self[y][x];
        let (from_type, from_owner) = if let Some(from) = from {
            (from.piece_type, from.owner)
        } else {
            return moves;
        };

        for iy in max(y - 1, 0)..min(y + 1, self.size - 1) {
            for ix in max(x - 1, 0)..min(x + 1, self.size - 1) {
                if (ix, iy) == (x, y) {
                    continue;
                }
                let to = self[iy][ix];
                if to.is_none_or(|to| {
                    from_owner != to.owner
                        && PieceType::resolve(from_type, to.piece_type)
                            .is_some_and(|t| t == from_type)
                }) {
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
