use othello::{
    agents::minimax::minimax,
    othello::{Game, Play},
};

fn main() {
    divan::main();
}

#[divan::bench(args = [3, 4, 5, 6, 7, 8, 9, 10])]
fn bench_minimax_start(depth: u32) {
    let game = Game::new();
    let best_move = minimax(game, depth);
    std::hint::black_box(best_move);
}

#[divan::bench(args = [3, 4, 5, 6, 7, 8, 9])]
fn bench_minimax_midgame(depth: u32) {
    let mut game = Game::new();
    // Play some moves to reach a midgame state
    let midgame_moves = [
        (3, 2),
        (2, 4),
        (3, 5),
        (4, 2),
        (5, 2),
        (2, 6),
        (3, 6),
        (3, 7),
        (1, 5),
        (0, 4),
    ];
    for &(x, y) in &midgame_moves {
        let play = Play::new(x, y);
        assert!(
            game.is_valid_play(play),
            "Invalid play {} in midgame setup",
            play
        );
        game.make_play(play);
    }

    let best_move = minimax(game, depth);
    std::hint::black_box(best_move);
}
