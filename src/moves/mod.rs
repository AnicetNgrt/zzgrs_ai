use crate::{game::Game, game_move::Move};

pub mod move_pass;
pub mod move_place_pawn;
pub mod move_displace_pawn;
pub mod move_kill_pawn;

#[derive(Clone, Debug, PartialEq)]
pub enum MoveType {
    MovePass,
    MovePlacePawn,
    MoveDisplacePawn,
    MoveKillPawn
}

macro_rules! generate_then_append {
    ($list: expr, $game: expr, [$($move_module:ident),*]) => {
        $(
            for el in $move_module::generate($game, $game.playing_plid).into_iter() {
                $list.push(Box::new(el));
            }
        )*
    };
}

pub fn generate_moves(game: &Game) -> Vec<Box<dyn Move>> {
    let mut moves: Vec<Box<dyn Move>> = vec![];

    generate_then_append!(moves, game, [
        move_pass,
        move_place_pawn,
        move_displace_pawn,
        move_kill_pawn
    ]);

    moves
}