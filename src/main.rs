use zzgrs::{
    coord::Coord,
    game::Game,
    game_move::Move,
    params::Params,
    player::Player,
    players::minimax::Minimax,
    tile::{OccupiedTile, Tile},
};

fn main() {
    let mut players: Vec<Box<dyn Player>> =
        vec![Box::new(Minimax { plid: 0 }), Box::new(Minimax { plid: 1 })];

    let mut game = Game::new(Params {
        turns: 20,
        players: 2,
        pawns: 3,
        width: 8,
        height: 11,
        apts: 3,
    });

    while !game.finished() {
        println!("Turn {} | Player {}", game.turn, game.playing_plid);
        let mut moves = Move::generate_all(&mut game);
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
                    Tile::OccupiedTile(OccupiedTile { plid, pwid }) => {
                        print!(" {}:{} ", plid, pwid)
                    }
                }
            }
            println!("");
        }
    }
}
