use rayon::prelude::*;

use super::*;

pub fn perft(game: &mut Game, depth: u8, print: bool) -> u128 {
    seq_perft(game, depth, print, &mut SearchEnv::new())
}

fn par_perft(game: &mut Game, depth: u8, print: bool) -> u128 {
    let moves = generate_moves(game);

    if depth == 1 {
        return moves.len() as u128;
    }

    moves.values().par_iter().map(|m| {
        let mut copy = game.clone();
        let mut envir = SearchEnv::new();

        make_move(&mut copy, &m, &mut envir);
        let r = seq_perft(&mut copy, depth - 1, false, &mut envir);

        if print {
            println!("{}{}: {}", SQUARE_STRINGS[m.from_square() as usize], SQUARE_STRINGS[m.to_square() as usize], r)
        }

        r
    }).sum()
}

fn seq_perft(game: &mut Game, depth: u8, print: bool, envir: &mut SearchEnv) -> u128 {
    let moves = generate_moves(game);

    if depth == 1 {
        return moves.len() as u128;
    }

    moves.values().iter().map(|m| {
        envir.ply += 1;

        make_move(game, &m, envir);

        let r = seq_perft(game, depth - 1, false, envir);

        unmake_move(game, m, envir);

        envir.ply -= 1;

        if print {
            println!("{}{}: {}", SQUARE_STRINGS[m.from_square() as usize], SQUARE_STRINGS[m.to_square() as usize], r)
        }

        r
    }).sum()
}