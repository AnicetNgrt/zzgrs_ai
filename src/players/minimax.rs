use crate::{player::Player, game::Game, game_move::Move};

pub struct Minimax {}

impl Player for Minimax {
    fn play(&mut self, game: &Game, moves: &Vec<Box<dyn Move>>) -> usize {
        0
    }
}

/*
fn minimax(state: &State, depth: i32, alpha: i32, beta: i32, maximizing_player: bool) -> EvaluatedState {
    // Check if we've reached the maximum depth or the game is over.
    if depth == 0 || state.is_game_over() {
        return EvaluatedState {
            state: state.clone(),
            score: evaluate_state(state),
        };
    }

    let mut evaluated_states: Vec<EvaluatedState> = Vec::new();

    // Generate all possible moves.
    let mut moves = state.generate_moves();

    // Sort the moves based on the evaluation score of their resulting states.
    moves.sort_by_key(|move| evaluate_state(&state.apply_move(move)));

    // Evaluate each move and choose the best one.
    for move in moves {
        let new_state = state.apply_move(move);

        let evaluated_state = minimax(
            &new_state,
            depth - 1,
            alpha,
            beta,
            !maximizing_player,
        );

        evaluated_states.push(evaluated_state);

        // Update alpha and beta based on the current player.
        if maximizing_player {
            alpha = alpha.max(evaluated_state.score);
        } else {
            beta = beta.min(evaluated_state.score);
        }

        // Check if we can prune the remaining moves.
        if beta <= alpha {
            break;
        }
    }

    // Choose the best evaluated state based on the current player.
    let best_evaluated_state = if maximizing_player {
        evaluated_states.iter().max_by_key(|state| state.score)
    } else {
        evaluated_states.iter().min_by_key(|state| state.score)
    };

    best_evaluated_state.unwrap().clone()
}
 */