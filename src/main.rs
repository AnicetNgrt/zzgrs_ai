use zzgrs::{players::minimax::Minimax, player::Player, game::Game, params::Params, moves::generate_moves, utils, coord::Coord, tile::{EmptyTile, OccupiedTile, Tile}};

fn main() {
    let mut players: Vec<Box<dyn Player>> = 
        vec![Box::new(Minimax{ plid: 0 }), Box::new(Minimax{ plid: 1 })];
    
    let mut game = Game::new(Params {
        turns: 20,
        players: 2,
        pawns: 3,
        width: 8,
        height: 11,
        apts: 2
    });

    // let mut selected_coords = vec![vec![false; game.params.width as usize]; game.params.height as usize];
    // for coord in utils::distant_coords(&game, Coord{ x:0, y:0 }, 4, true, false) {
    //     println!("{:?}", coord);
    //     selected_coords[coord.y as usize][coord.x as usize] = true;
    // }
    // for y in 0..game.params.height as usize {
    //     for x in 0..game.params.width as usize {
    //         if selected_coords[y][x] {
    //             print!("[]");
    //         } else {
    //             print!(",'");
    //         }
    //     }
    //     println!("");
    // }

    while !game.finished() {
        println!("Turn {} | Player {}", game.turn, game.playing_plid);
        let mut moves = generate_moves(&mut game);
        println!("{}", moves.len());
        let index = players[game.playing_plid].play(&mut game);
        let mut m = moves.get_mut(index).expect("");
        game.apply_move(&mut m);

        for y in 0..game.params.height {
            for x in 0..game.params.width {
                let coord = Coord { x, y };
                let tile = game.get_tile(&coord);
                match tile {
                    Tile::EmptyTile(_) => print!(" ... "),
                    Tile::OccupiedTile(OccupiedTile{ plid, pwid }) => print!(" {}:{} ", plid, pwid)
                }
            }
            println!("");
        } 
    }
}
