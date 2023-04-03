#[derive(Clone, Debug)]
pub enum Tile {
    EmptyTile(EmptyTile),
    OccupiedTile(OccupiedTile),
}

#[derive(Clone, Debug)]
pub struct EmptyTile;

#[derive(Clone, Debug)]
pub struct OccupiedTile {
    pub plid: usize,
    pub pwid: usize,
}
