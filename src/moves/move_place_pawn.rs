use crate::{
    coord::Coord,
    game::Game,
    game_move::Move,
    pawn::{Pawn, PlacedPawn},
    tile::{OccupiedTile, Tile},
    utils,
};

use super::MoveType;

#[derive(Clone, Debug)]
pub struct MovePlacePawn {
    plid: usize,
    pwid: usize,
    coord: Coord,
    previous_pawn: Option<Pawn>,
    previous_tile: Option<Tile>,
}

impl MovePlacePawn {
    pub fn new(plid: usize, pwid: usize, coord: Coord) -> Self {
        MovePlacePawn {
            coord,
            pwid,
            plid,
            previous_pawn: None,
            previous_tile: None,
        }
    }

    pub fn generate(game: &Game, plid: usize) -> Vec<Move> {
        utils::staged_pawns_pwids(game, plid)
            .flat_map(|(pwid, _)| {
                utils::empty_coords(game).map(move |coord| MovePlacePawn::new(plid, pwid, coord))
            })
            .map(|m| Move::MovePlacePawn(m))
            .collect()
    }
}

impl MoveType for MovePlacePawn {
    fn apply(&mut self, game: &mut crate::game::Game) {
        self.previous_pawn = Some(game.get_pawn(self.plid, self.pwid));
        game.set_pawn(
            self.plid,
            self.pwid,
            Pawn::PlacedPawn(PlacedPawn { coord: self.coord }),
        );
        self.previous_tile = Some(game.get_tile(&self.coord));
        game.set_tile(
            self.coord,
            Tile::OccupiedTile(OccupiedTile {
                plid: self.plid,
                pwid: self.pwid,
            }),
        )
    }

    fn rollback(&mut self, game: &mut crate::game::Game) {
        game.set_tile(self.coord, self.previous_tile.as_ref().expect("").clone());
        game.set_pawn(
            self.plid,
            self.pwid,
            self.previous_pawn.as_ref().expect("").clone(),
        );
    }

    fn regenerate(&self) -> Self {
        Self::new(self.plid, self.pwid, self.coord)
    }
}
