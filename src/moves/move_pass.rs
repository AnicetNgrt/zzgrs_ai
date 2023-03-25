use crate::{game_move::{Move, MoveAttribute, MoveDescription}, game::Game};

use super::MoveType;

#[derive(Default, Clone)]
pub struct MovePass {
    previous_plid: Option<usize>,
    previous_turn: Option<usize>,
    previous_apts: Option<u8>
}

impl Move for MovePass {
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

    fn describe(&self, game: &Game) -> MoveDescription {
        MoveDescription(MoveType::MovePass, Vec::<MoveAttribute>::new())
    }
}

pub fn generate(_game: &Game, plid: usize) -> Vec<MovePass> {
    return vec![MovePass::default()]
}