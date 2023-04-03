use crate::game::Game;

pub mod move_displace_pawn;
pub mod move_kill_pawn;
pub mod move_pass;
pub mod move_place_pawn;

pub trait MoveType {
    fn apply(&mut self, game: &mut Game);
    fn rollback(&mut self, game: &mut Game);
    fn regenerate(&self) -> Self;
}