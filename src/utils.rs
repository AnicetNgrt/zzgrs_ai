use std::collections::{VecDeque, HashMap};

use crate::{game::Game, pawn::{Pawn, PlacedPawn, StagedPawn, DeadPawn}, coord::Coord, tile::{Tile}};

macro_rules! Iter {
    [ $type:ty ] => {
        impl Iterator<Item=$type> + '_
    };
}

pub fn plids(game: &Game) -> Iter![usize] {
    0..game.params.players
}

pub fn coords(game: &Game) -> Iter![Coord] {
    (0i8..game.params.width).into_iter()
        .flat_map(|x| (0i8..game.params.height).map(move |y| (x, y)))
        .map(|(x, y)| Coord { x, y })
}

pub fn empty_coords(game: &Game) -> Iter![Coord] {
    coords(game).filter(|coord| match game.get_tile(coord) {
        Tile::EmptyTile(_) => true,
        _ => false
    })
}

pub fn dead_pawns_pwids(game: &Game, plid: usize) -> Iter![(usize, &DeadPawn)] {
    game.pawns.get(plid).expect("")
        .iter()
        .enumerate()
        .filter_map(|(i, p)| match p {
            Pawn::DeadPawn(p) => Some((i, p)),
            _ => None
        })
}

pub fn dead_pawns_count(game: &Game, plid: usize) -> i32 {
    game.pawns.get(plid).expect("")
        .iter()
        .filter(|p| match p {
            Pawn::DeadPawn(_) => true,
            _ => false
        })
        .count().try_into().expect("")
}

pub fn staged_pawns_pwids(game: &Game, plid: usize) -> Iter![(usize, &StagedPawn)] {
    game.pawns.get(plid).expect("")
        .iter()
        .enumerate()
        .filter_map(|(i, p)| match p {
            Pawn::StagedPawn(p) => Some((i, p)),
            _ => None
        })
}

pub fn staged_pawns_count(game: &Game, plid: usize) -> i32 {
    game.pawns.get(plid).expect("")
        .iter()
        .filter(|p| match p {
            Pawn::StagedPawn(_) => true,
            _ => false
        })
        .count().try_into().expect("")
}

pub fn placed_pawns_pwids(game: &Game, plid: usize) -> Iter![(usize, &PlacedPawn)] {
    game.pawns.get(plid).expect("")
        .iter()
        .enumerate()
        .filter_map(|(i, p)| match p {
            Pawn::PlacedPawn(p) => Some((i, p)),
            _ => None
        })
}

pub fn placed_pawns_count(game: &Game, plid: usize) -> i32 {
    game.pawns.get(plid).expect("")
        .iter()
        .filter(|p| match p {
            Pawn::PlacedPawn(_) => true,
            _ => false
        })
        .count().try_into().expect("")
}

// pub fn distant_coords(game: &Game, center: Coord, d: i8, wrap: bool) -> Iter![Coord] {
//     coords(game).filter(move |coord| {
//         let current_d = match wrap {
//             true => center.distance_wrap(coord, game.params.width, game.params.height),
//             false => center.distance_nowrap(coord)
//         };
//         current_d <= d 
//     })
// }

pub fn distant_coords(game: &Game, center: Coord, d: i8, wrap: bool, exclusive: bool) -> Iter![Coord] {
    let directions = vec![
        Coord { x: -1, y: 0 },
        Coord { x: 0, y: 1 },
        Coord { x: 1, y: 0 },
        Coord { x: 0, y: -1 }
    ];

    let mut fifo: VecDeque<Coord> = VecDeque::new();
    let mut visited: HashMap<Coord, bool> = HashMap::new();
    fifo.push_front(center);
    visited.insert(center, !exclusive || d == 0);

    loop {
        let current = if let Some(coord) = fifo.pop_back() {
            coord
        } else {
            break;
        };
        'inner: for i in 0..directions.len() {
            let direction = directions[i];
            let visiting = if wrap {
                current.add_wrap(&direction, game.params.width, game.params.height)
            } else {
                if let Some(coord) = current.add_nowrap(&direction, game.params.width, game.params.height) {
                    coord
                } else {
                    continue 'inner;
                }
            };

            let distance = if wrap {
                visiting.distance_wrap(&center, game.params.width, game.params.height)
            } else {
                visiting.distance_nowrap(&center, game.params.width, game.params.height)
            };

            if wrap && distance > d {
                break;
            }
            if !wrap && distance > d {
                continue;
            }

            if !visited.contains_key(&visiting) {
                visited.insert(visiting, !exclusive || d == distance);
                fifo.push_front(visiting);
            };
        }
    };

    visited.into_iter()
        .filter_map(|(coord, included)| if included { Some(coord) } else { None })
}

// fn max_new_tiles(d: i8) -> i32 {
//     if d == 0 {
//         1
//     } else {
//         (d as i32) *4
//     }
// }

// fn max_tiles_count(d: i8) -> i32 {
//     if d < 0 {
//         0
//     }
//     else if d == 0 {
//         max_new_tiles(0)
//     } else {
//         max_new_tiles(d) + max_new_tiles(d-1)
//     }
// }