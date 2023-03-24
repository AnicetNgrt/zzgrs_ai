pub enum Tile {
    EmptyTile(EmptyTile),
    OccupiedTile(OccupiedTile)
}

pub struct EmptyTile;

pub struct OccupiedTile {
    pub plid: usize,
    pub pwid: usize
}