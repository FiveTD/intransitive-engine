pub trait GameState {
    type Move;

    fn legal_moves(&self) -> Vec<Self::Move>;
    fn apply_move(&self, mv: &Self::Move) -> Self;

    fn is_terminal(&self) -> bool;

    /// The score of the current board state from the perspective of the active player.
    fn evaluate(&self) -> i32;
}
