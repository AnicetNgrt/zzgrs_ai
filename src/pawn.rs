use crate::coord::Coord;

#[derive(Clone, Debug)]
pub enum Pawn {
    PlacedPawn(PlacedPawn),
    StagedPawn(StagedPawn),
    DeadPawn(DeadPawn)
}

#[derive(Clone, Debug)]
pub struct PlacedPawn {
    pub coord: Coord
}

#[derive(Clone, Debug)]
pub struct StagedPawn;

#[derive(Clone, Debug)]
pub struct DeadPawn;