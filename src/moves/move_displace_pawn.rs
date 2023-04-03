use crate::{
    coord::Coord,
    game::Game,
    game_move::Move,
    pawn::{Pawn, PlacedPawn},
    tile::{EmptyTile, OccupiedTile, Tile},
    utils,
};

use super::MoveType;

#[derive(Clone, Debug)]
pub struct MoveDisplacePawn {
    plid: usize,
    pwid: usize,
    coord: Coord,
    previous_coord: Option<Coord>,
}

impl MoveDisplacePawn {
    pub fn new(plid: usize, pwid: usize, coord: Coord) -> Self {
        MoveDisplacePawn {
            coord,
            pwid,
            plid,
            previous_coord: None,
        }
    }

    pub fn generate(game: &Game, plid: usize) -> Vec<Move> {
        if game.apts(plid) < 1 || utils::staged_pawns_count(game, plid) > 0 {
            return Vec::new();
        }

        utils::placed_pawns_pwids(game, plid)
            .flat_map(|(pwid, placed_pawn)| {
                utils::distant_coords(game, placed_pawn.coord, 4, true, false).filter_map(
                    move |coord| match game.get_tile(&coord) {
                        Tile::EmptyTile(_) => Some(MoveDisplacePawn::new(plid, pwid, coord)),
                        _ => None,
                    },
                )
            })
            .map(|m| Move::MoveDisplacePawn(m))
            .collect()
    }
}

impl MoveType for MoveDisplacePawn {
    fn apply(&mut self, game: &mut Game) {
        let previous_coord = match game.get_pawn(self.plid, self.pwid) {
            Pawn::PlacedPawn(PlacedPawn { coord: c }) => c,
            _ => panic!(),
        };
        self.previous_coord = Some(previous_coord);
        game.set_pawn(
            self.plid,
            self.pwid,
            Pawn::PlacedPawn(PlacedPawn { coord: self.coord }),
        );
        game.set_tile(previous_coord, Tile::EmptyTile(EmptyTile));
        game.set_tile(
            self.coord,
            Tile::OccupiedTile(OccupiedTile {
                plid: self.plid,
                pwid: self.pwid,
            }),
        );
        game.rem_apts(self.plid, 1);
    }

    fn rollback(&mut self, game: &mut Game) {
        game.add_apts(self.plid, 1);
        let previous_coord = self.previous_coord.expect("");
        game.set_tile(self.coord, Tile::EmptyTile(EmptyTile));
        game.set_tile(
            previous_coord,
            Tile::OccupiedTile(OccupiedTile {
                plid: self.plid,
                pwid: self.pwid,
            }),
        );
        game.set_pawn(
            self.plid,
            self.pwid,
            Pawn::PlacedPawn(PlacedPawn {
                coord: previous_coord,
            }),
        );
    }

    fn regenerate(&self) -> Self {
        Self::new(self.plid, self.pwid, self.coord)
    }
}
