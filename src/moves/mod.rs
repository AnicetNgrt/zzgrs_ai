use crate::{game::Game, game_move::Move};

pub mod move_pass;

macro_rules! generate_then_append {
    ($list: expr, $game: expr, [$($move_module:ident),*]) => {
        $(
            for el in $move_module::generate($game).into_iter() {
                $list.push(Box::new(el));
            }
        )*
    };
}



pub fn generate_moves(game: &Game) -> Vec<Box<dyn Move>> {
    let mut moves: Vec<Box<dyn Move>> = vec![];

    generate_then_append!(moves, game, [
        move_pass
    ]);

    moves
}