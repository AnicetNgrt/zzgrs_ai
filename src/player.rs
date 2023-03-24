use crate::{game::Game, game_move::Move};

pub trait Player {    
    fn play(&mut self, game: &Game, moves: &Vec<Box<dyn Move>>) -> usize;
}