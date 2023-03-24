use zzgrs::{players::minimax::Minimax, player::Player, game::Game, params::Params, moves::generate_moves};

fn main() {
    let mut players: Vec<Box<dyn Player>> = 
        vec![Box::new(Minimax{}), Box::new(Minimax{})];
    
    let mut game = Game::new(Params {
        turns: 2,
        players: 2,
    });

    while !game.finished() {
        println!("Turn {} | Player {}", game.turn, game.playing_plid);
        let mut moves = generate_moves(&mut game);
        let index = players[game.playing_plid].play(&mut game, &moves);
        let mut m = moves.get_mut(index).expect("");
        game.apply_move(&mut m);
    }
}
