use crate::GameState;

fn negamax<T: GameState>(state: &T, depth: usize, mut alpha: i32, beta: i32) -> i32 {
    if depth == 0 || state.is_terminal() {
        return state.evaluate();
    }

    let mut value = i32::MIN;

    for mv in state.legal_moves() {
        let child = state.apply_move(&mv);

        let score = -negamax(&child, depth - 1, -beta, -alpha);

        value = value.max(score);
        alpha = alpha.max(value);

        if alpha >= beta {
            break;
        }
    }

    value
}

pub fn find_best_move<T: GameState>(state: &T, depth: usize) -> Option<T::Move> {
    if depth == 0 {
        return None;
    }

    state
        .legal_moves()
        .into_iter()
        .map(|mv| {
            let next_state = state.apply_move(&mv);
            let score = negamax(&next_state, depth - 1, i32::MIN + 1, i32::MAX);

            (mv, score)
        })
        .max_by_key(|(_, score)| *score)
        .map(|(mv, _)| mv)
}
