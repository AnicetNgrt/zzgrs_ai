use crate::{game::Game};

pub trait Player {    
    fn play(&mut self, game: &Game) -> usize;
}