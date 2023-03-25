use crate::{game::Game, coord::Coord, tile::Tile, pawn::Pawn, moves::{MoveType}};

#[derive(Debug)]
pub struct MoveDescription(pub MoveType, pub Vec<MoveAttribute>);

#[derive(Clone, Debug)]
pub struct MoveAttribute {
    pub name: String,
    pub value: MoveAttributeValue
}

#[derive(Clone, Debug)]
pub enum MoveAttributeValue {
    Plid(usize),
    Pawn(usize, usize, Pawn),
    Coord(Coord, Tile),
}

pub trait Move {
    fn apply(&mut self, game: &mut Game);
    fn rollback(&mut self, game: &mut Game);
    fn describe(&self, game: &Game) -> MoveDescription;
}

pub fn expect_attr_plid<'a>(name: &str, attrs: &'a Vec<MoveAttribute>) -> &'a usize {
    for attr in attrs.iter() {
        if attr.name == name {
            let value = &attr.value;
            match value {
                MoveAttributeValue::Plid(plid) => return &plid,
                _ => {}
            }
        }
    };
    panic!()
}

pub fn expect_attr_pawn<'a>(name: &str, attrs: &'a Vec<MoveAttribute>) -> (&'a usize, &'a usize, &'a Pawn) {
    for attr in attrs.iter() {
        if attr.name == name {
            let value = &attr.value;
            match value {
                MoveAttributeValue::Pawn(plid, pwid, pawn) => return (&plid, &pwid, &pawn),
                _ => {}
            }
        }
    };
    panic!()
}

pub fn expect_attr_coord<'a>(name: &str, attrs: &'a Vec<MoveAttribute>) -> (&'a Coord, &'a Tile) {
    for attr in attrs.iter() {
        if attr.name == name {
            let value = &attr.value;
            match value {
                MoveAttributeValue::Coord(coord, tile) => return (&coord, &tile),
                _ => {}
            }
        }
    };
    panic!()
}