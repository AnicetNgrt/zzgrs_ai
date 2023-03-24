use crate::{game_move::Move, game::Game};

#[derive(Default, Clone)]
pub struct MovePass {
    previous_plid: Option<usize>,
    previous_turn: Option<usize>
}

impl Move for MovePass {
    fn apply(&mut self, game: &mut crate::game::Game) {
        self.previous_turn = Some(game.turn);
        game.turn += 1;
        self.previous_plid = Some(game.playing_plid);
        game.playing_plid = (game.playing_plid + 1) % game.params.players;
    }

    fn rollback(&mut self, game: &mut crate::game::Game) {
        game.playing_plid = self.previous_plid.expect("");
        game.turn = self.previous_turn.expect("");
    }
}

pub fn generate(_game: &Game) -> Vec<MovePass> {
    return vec![MovePass::default()]
}