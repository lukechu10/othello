use othello::{agents::minimax::minimax, othello::Game};

fn main() {
    divan::main();
}

#[divan::bench(args = [3, 4, 5, 6, 7, 8, 9, 10])]
fn bench_minimax(depth: u32) {
    let game = Game::new();
    let best_move = minimax(game, depth);
    std::hint::black_box(best_move);
}
