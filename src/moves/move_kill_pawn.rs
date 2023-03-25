use std::collections::HashSet;

use crate::{game_move::{Move, MoveAttribute, MoveAttributeValue, MoveDescription}, game::Game, pawn::{Pawn, PlacedPawn, DeadPawn}, tile::{OccupiedTile, Tile, EmptyTile}, utils, coord::Coord};

use super::MoveType;

#[derive(Clone)]
pub struct MoveKillPawn {
    plid: usize,
    eplid: usize,
    epwid: usize,
    previous_epawn: Option<Pawn>,
    previous_ecoord: Option<Coord>,
    previous_etile: Option<Tile>
}

impl MoveKillPawn {
    fn new(plid: usize, eplid: usize, epwid: usize) -> Self {
        MoveKillPawn {  
            plid, 
            epwid, 
            eplid, 
            previous_epawn: None, 
            previous_ecoord: None, 
            previous_etile: None 
        }
    }
}

impl Move for MoveKillPawn {
    fn apply(&mut self, game: &mut crate::game::Game) {
        self.previous_epawn = Some(game.get_pawn(self.eplid, self.epwid));
        let previous_coord = match self.previous_epawn {
            Some(Pawn::PlacedPawn(PlacedPawn{coord: c})) => c,
            _ => panic!()
        };
        self.previous_ecoord = Some(previous_coord);
        game.set_pawn(self.eplid, self.epwid, Pawn::DeadPawn(DeadPawn));
        self.previous_etile = Some(game.get_tile(&previous_coord));
        game.set_tile(previous_coord, Tile::EmptyTile(EmptyTile));
        game.rem_apts(self.plid, 1);
    }

    fn rollback(&mut self, game: &mut crate::game::Game) {
        game.add_apts(self.plid, 1);
        game.set_tile(self.previous_ecoord.expect(""), Tile::OccupiedTile(OccupiedTile { plid: self.eplid, pwid: self.epwid }));
        game.set_pawn(self.eplid, self.epwid, self.previous_epawn.as_ref().expect("").clone());
    }

    fn describe(&self, game: &Game) -> MoveDescription {
        MoveDescription(MoveType::MoveKillPawn, vec![
            MoveAttribute{ 
                name: "Player".to_string(), 
                value: MoveAttributeValue::Plid(self.plid) 
            },
            MoveAttribute{ 
                name: "Killed pawn".to_string(), 
                value: MoveAttributeValue::Pawn(self.eplid, self.epwid, game.get_pawn(self.eplid, self.epwid)) 
            }
        ])
    }
}

pub fn generate(game: &Game, plid: usize) -> Vec<MoveKillPawn> {
    if game.apts(plid) < 1 
        || utils::staged_pawns_count(game, plid) > 0 {
        return Vec::new()
    }
    let mut duplicates = HashSet::<Coord>::new();
    utils::placed_pawns_pwids(game, plid)
        .flat_map(|(_, placed_pawn)| {
            utils::distant_coords(game, placed_pawn.coord, 1, false, true)
                .filter(|coord| *coord != placed_pawn.coord)
        })
        .filter_map(|coord| {
            if duplicates.contains(&coord) {
                return None;
            }
            duplicates.insert(coord);
            match game.get_tile(&coord) {
                Tile::OccupiedTile(OccupiedTile { plid: eplid, pwid: epwid }) => if eplid != plid {
                    Some(MoveKillPawn::new(plid, eplid, epwid))
                } else {
                    None
                },
                _ => None
            }
        })
        .collect()
}