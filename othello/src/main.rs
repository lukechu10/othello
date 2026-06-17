use othello::mcts::*;
use othello::othello::*;

fn main() {
    let mut game = Game::new();

    println!("{}", game);
    while game.game_state() == Player::InProgress {
        let play = if game.player_to_move == Player::Black {
            let mut mcts_agent = Mcts::new(game);
            let search_res = mcts_agent.run_search_time_budget(100);
            println!("Black: {} games simulated", search_res.search_iterations);

            mcts_agent.best_play()
        } else {
            let mut mcts_agent = Mcts::new(game);
            let search_res = mcts_agent.run_search_iterations_budget(15000);
            println!("White: {} games simulated", search_res.search_iterations);

            mcts_agent.best_play()
        };
        game.make_play(play);
    }

    println!("{:?} wins", game.game_state());
}
