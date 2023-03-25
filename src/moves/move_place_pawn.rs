use crate::{game_move::{Move, MoveAttribute, MoveAttributeValue, MoveDescription}, game::Game, coord::Coord, pawn::{Pawn, PlacedPawn}, tile::{OccupiedTile, Tile}, utils};

use super::MoveType;

#[derive(Clone)]
pub struct MovePlacePawn {
    plid: usize,
    pwid: usize,
    coord: Coord,
    previous_pawn: Option<Pawn>,
    previous_tile: Option<Tile>
}

impl MovePlacePawn {
    fn new(plid: usize, pwid: usize, coord: Coord) -> Self {
        MovePlacePawn { coord, pwid, plid, previous_pawn: None, previous_tile: None }
    }
}

impl Move for MovePlacePawn {
    fn apply(&mut self, game: &mut crate::game::Game) {
        self.previous_pawn = Some(game.get_pawn(self.plid, self.pwid));
        game.set_pawn(self.plid, self.pwid, Pawn::PlacedPawn(PlacedPawn{ coord: self.coord }));
        self.previous_tile = Some(game.get_tile(&self.coord));
        game.set_tile(self.coord, Tile::OccupiedTile(OccupiedTile { plid: self.plid, pwid: self.pwid }))
    }

    fn rollback(&mut self, game: &mut crate::game::Game) {
        game.set_tile(self.coord, self.previous_tile.as_ref().expect("").clone());
        game.set_pawn(self.plid, self.pwid, self.previous_pawn.as_ref().expect("").clone());
    }

    fn describe(&self, game: &Game) -> MoveDescription {
        MoveDescription(MoveType::MovePlacePawn, vec![
            MoveAttribute{ 
                name: "Player".to_string(), 
                value: MoveAttributeValue::Plid(self.plid) 
            },
            MoveAttribute{ 
                name: "Placed pawn".to_string(), 
                value: MoveAttributeValue::Pawn(self.plid, self.pwid, game.get_pawn(self.plid, self.pwid)) 
            },
            MoveAttribute{ 
                name: "Placed position".to_string(), 
                value: MoveAttributeValue::Coord(self.coord, game.get_tile(&self.coord)) 
            },
        ])
    }
}

pub fn generate(game: &Game, plid: usize) -> Vec<MovePlacePawn> {
    utils::staged_pawns_pwids(game, plid)
        .flat_map(|(pwid, _)| {
            utils::empty_coords(game)
                .map(move |coord| MovePlacePawn::new(plid, pwid, coord))
        })
        .collect()
}