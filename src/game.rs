use crate::{params::Params, game_move::Move};

pub struct Game {
    pub cached_moves: Option<Vec<Box<dyn Move>>>,
    pub params: Params,
    pub turn: usize,
    pub playing_plid: usize
}

impl Game {
    pub fn new(params: Params) -> Self {
        Self {
            params,
            cached_moves: None,
            turn: 0,
            playing_plid: 0
        }
    }

    pub fn finished(&self) -> bool {
        self.turn > self.params.turns
    }

    pub fn moves(&self) -> Vec<&Box<dyn Move>> {
        let mut res = vec![];
        if let Some(moves) = &self.cached_moves {
            for m in moves.iter() {
                res.push(m);
            }
        }
        res
    }

    pub fn apply_move(&mut self, m: &mut Box<dyn Move>) {
        m.apply(self);
    }

    pub fn rollback_move(&mut self, m: &mut Box<dyn Move>) {
        m.rollback(self);
    }
}