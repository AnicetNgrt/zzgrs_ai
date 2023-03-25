use std::i32::{MIN, MAX};

use rand::{Rng};

use crate::{player::Player, game::Game, game_move::{Move, MoveDescription, expect_attr_coord}, moves::{generate_moves, move_kill_pawn::{MoveKillPawn, generate, self}, move_displace_pawn::MoveDisplacePawn, move_place_pawn::MovePlacePawn, move_pass::MovePass, MoveType}, utils};

pub struct GranularMinimax {
    pub plid: usize
}

impl Player for GranularMinimax {
    fn play(&mut self, game: &Game) -> usize {
        let moves = generate_moves(game);
        let descriptions: Vec<MoveDescription> = moves.iter().map(|m| m.describe(game)).collect();
        
        let (_, maybe_index) = self.minimax(
            &mut game.clone(), 
            8, 
            MIN, 
            MAX, 
            true,
            moves
        );

        println!("Chosen move: {:?}", descriptions[maybe_index.unwrap()]);

        if let Some(index) = maybe_index {
            index
        } else {
            0
        }
    }
}

impl GranularMinimax {
    fn score(&self, game: &Game) -> i32 {
        let opp_plid = (game.playing_plid + 1) % game.params.players;
        let advancement = utils::dead_pawns_count(game, opp_plid) * 10;
        let opportunity = move_kill_pawn::generate(game, game.playing_plid).len() as i32;
        let dangerosity = move_kill_pawn::generate(game, opp_plid).len() as i32;
        advancement + opportunity - dangerosity
    }

    fn rank_move_types(&self, move_type: &MoveType) -> i32 {
        match move_type {
            MoveType::MoveKillPawn => 3,
            MoveType::MoveDisplacePawn => 2,
            MoveType::MovePlacePawn => 1,
            MoveType::MovePass => 0
        }
    }

    fn minimax(
        &self, 
        game: &mut Game, depth: i32, 
        alpha: i32, 
        beta: i32, 
        maximizing: bool,
        mut moves: Vec<Box<dyn Move>>
    ) -> (i32, Option<usize>) {
        if depth == 0 || game.finished() {
            return (self.score(game), None);
        }

        let mut rng = rand::thread_rng();

        moves.sort_by(|a, b| {
            let MoveDescription(type_a, attrs_a) = a.describe(game);
            let MoveDescription(type_b, attrs_b) = b.describe(game);
            if type_a == type_b {
                if type_a == MoveType::MoveDisplacePawn {
                    let (_coord_a, _tile_a) = expect_attr_coord("Displaced coord", &attrs_a);
                    let (_coord_b, _tile_b) = expect_attr_coord("Displaced coord", &attrs_b);
                    // compare coords   
                }
            }
            self.rank_move_types(&type_a).cmp(&self.rank_move_types(&type_b))
        });
        
        let best_deep_score = if maximizing { MIN } else { MAX };
        let mut best_deep_move: Option<usize> = None;
        let mut alpha = alpha;
        let mut beta = beta;
        
        for (i, m) in moves.iter_mut().enumerate().rev().take(6) {
            if let None = best_deep_move {
                best_deep_move = Some(i);
            }

            game.apply_move(m);
            let (deep_score, _) = self.minimax(
                game, 
                depth-1, 
                alpha, 
                beta, 
                self.plid == game.playing_plid, 
                generate_moves(game)
            );
            game.rollback_move(m);

            if (maximizing && deep_score > best_deep_score)
                || (!maximizing && deep_score < best_deep_score) {
                best_deep_move = Some(i);
            } else if rng.gen_range(0..100) > 50 && deep_score == best_deep_score {
                best_deep_move = Some(i);
            }

            if maximizing {
                alpha = alpha.max(best_deep_score);
            } else {
                beta = beta.min(best_deep_score);
            }

            if (maximizing && best_deep_score >= beta) 
                || (!maximizing && best_deep_score <= alpha) {
                break;
            }
        }

        (best_deep_score, best_deep_move)
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