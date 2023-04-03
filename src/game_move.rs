use crate::{
    game::Game,
    moves::{
        move_displace_pawn::MoveDisplacePawn, move_kill_pawn::MoveKillPawn, move_pass::MovePass,
        move_place_pawn::MovePlacePawn,
    },
};
use crate::moves::MoveType;

macro_rules! Move {
    ([$($move_type:ident),*]) => {
        #[derive(Debug)]
        pub enum Move {
            $(
                $move_type($move_type),
            )*
        }

        impl Move {
            pub fn apply(&mut self, game: &mut Game) {
                match self {
                    $(
                        Move::$move_type(m) => m.apply(game),
                    )*
                }
            }

            pub fn rollback(&mut self, game: &mut Game) {
                match self {
                    $(
                        Move::$move_type(m) => m.rollback(game),
                    )*
                }
            }

            pub fn regenerate(&self) -> Self {
                match self {
                    $(
                        Move::$move_type(m) => Move::$move_type(m.regenerate()),
                    )*
                }
            }

            pub fn generate_all(game: &Game) -> Vec<Self> {
                let mut moves: Vec<Move> = vec![];

                $(
                moves.append(&mut $move_type::generate(game, game.playing_plid));
                )*

                moves
            }
        }
    };
}

Move!([MoveKillPawn, MoveDisplacePawn, MovePlacePawn, MovePass]);

pub fn regenerate_all(moves: &Vec<Move>) -> Vec<Move> {
    moves.iter().map(|m| m.regenerate()).collect()
}
