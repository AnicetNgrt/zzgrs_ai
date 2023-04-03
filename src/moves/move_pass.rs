use crate::{game::Game, game_move::Move};

use super::MoveType;

#[derive(Default, Clone, Debug)]
pub struct MovePass {
    previous_plid: Option<usize>,
    previous_turn: Option<usize>,
    previous_apts: Option<u8>,
}

impl MovePass {
    pub fn generate(_game: &Game, _plid: usize) -> Vec<Move> {
        return vec![Move::MovePass(MovePass::default())];
    }
}

impl MoveType for MovePass {
    fn apply(&mut self, game: &mut crate::game::Game) {
        self.previous_turn = Some(game.turn);
        game.turn += 1;
        self.previous_apts = Some(game.apts(game.playing_plid));
        game.set_apts(game.playing_plid, game.params.apts);
        self.previous_plid = Some(game.playing_plid);
        game.playing_plid = (game.playing_plid + 1) % game.params.players;
    }

    fn rollback(&mut self, game: &mut crate::game::Game) {
        game.playing_plid = self.previous_plid.expect("");
        game.set_apts(game.playing_plid, self.previous_apts.expect(""));
        game.turn = self.previous_turn.expect("");
    }

    fn regenerate(&self) -> Self {
        Self::default()
    }
}
