use crate::coord::Coord;

pub enum Pawn {
    PlacedPawn(PlacedPawn),
    StagedPawn(StagedPawn),
    DeadPawn(DeadPawn)
}

pub struct PlacedPawn {
    pub coord: Coord
}

pub struct StagedPawn;

pub struct DeadPawn;