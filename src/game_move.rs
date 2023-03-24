use crate::game::Game;

pub trait Move {
    fn apply(&mut self, game: &mut Game);
    fn rollback(&mut self, game: &mut Game);
}