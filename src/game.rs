use std::collections::HashMap;

use crate::{params::Params, game_move::Move, pawn::{Pawn, StagedPawn}, coord::Coord, tile::{Tile, OccupiedTile, EmptyTile}, utils};

#[derive(Clone)]
pub struct Game {
    pub params: Params,
    pub turn: usize,
    pub playing_plid: usize,
    pub pawns: Vec<Vec<Pawn>>,
    pub tiles: HashMap<Coord, Tile>,
    pub apts: Vec<u8>
}

impl Game {
    pub fn new(params: Params) -> Self {
        let pawns = Self::initial_pawns(&params);
        let apts = vec![0; params.players];
        
        Self {
            params,
            turn: 0,
            playing_plid: 0,
            pawns,
            tiles: HashMap::new(),
            apts,
        }
    }

    pub fn initial_pawns(params: &Params) -> Vec<Vec<Pawn>> {
        let mut pawns = Vec::new();
        for _ in 0..params.players {
            let mut player_pawns: Vec<Pawn> = Vec::new();
            for _ in 0..params.pawns {
                player_pawns.push(Pawn::StagedPawn(StagedPawn));
            }
            pawns.push(player_pawns);
        }
        pawns
    }

    pub fn apts(&self, plid: usize) -> u8 {
        self.apts[plid]
    }

    pub fn set_apts(&mut self, plid: usize, amount: u8) {
        self.apts[plid] = amount;
    }

    pub fn add_apts(&mut self, plid: usize, amount: u8) {
        self.apts[plid] += amount;
    }

    pub fn rem_apts(&mut self, plid: usize, amount: u8) {
        self.apts[plid] -= amount;
    }

    pub fn get_tile(&self, coord: &Coord) -> Tile {
        if let Some(tile) = self.tiles.get(coord) {
            tile.clone()
        } else {
            Tile::EmptyTile(EmptyTile)
        }
    }

    pub fn set_tile(&mut self, coord: Coord, tile: Tile) {
        self.tiles.insert(coord, tile);
    }

    pub fn get_pawn(&self, plid: usize, pwid: usize) -> Pawn {
        self.pawns[plid][pwid].clone()
    }

    pub fn set_pawn(&mut self, plid: usize, pwid: usize, pawn: Pawn) {
        self.pawns[plid][pwid] = pawn;
    }

    pub fn finished(&self) -> bool {
        let mut dead_players_count = 0;
        for plid in 0..self.params.players {
            if utils::dead_pawns_count(self, plid) == self.params.pawns as i32 {
                dead_players_count += 1;
            }
            if dead_players_count >= self.params.players - 1 {
                return true;
            }
        }
        self.turn > self.params.turns
    }

    pub fn apply_move(&mut self, m: &mut Box<dyn Move>) {
        m.apply(self);
    }

    pub fn rollback_move(&mut self, m: &mut Box<dyn Move>) {
        m.rollback(self);
    }
}