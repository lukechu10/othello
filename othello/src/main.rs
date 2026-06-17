use othello::agents::*;
use othello::othello::*;

#[allow(unused_variables)]
fn main() {
    let time_budget_ms = 50;

    let mut black_wins = 0;
    let mut white_wins = 0;
    let mut ties = 0;

    let mcts_easy = mcts::MctsAgent {
        max_iterations: 300,
    };
    let mcts_hard = mcts::MctsAgent {
        max_iterations: 10000,
    };
    let mcts_eval_easy = mcts_eval::MctsAgent {
        max_iterations: 300,
    };
    let mcts_eval_hard = mcts_eval::MctsAgent {
        max_iterations: 100000,
    };

    let minimax_easy = minimax::MinimaxAgent { max_depth: 5 };
    let minimax_hard = minimax::MinimaxAgent { max_depth: 7 };

    let mcts_mr_easy = mcts_mr::MctsAgent {
        max_iterations: 300,
    };
    let mcts_mr_hard = mcts_mr::MctsAgent {
        max_iterations: 10000,
    };

    let random = random::RandomAgent;

    for _ in 0..100 {
        let winner = Matchup::new(
            // random,
            // mcts_easy,
            // mcts_hard,
            // minimax::MinimaxAgent { max_depth: 5 },
            minimax::MinimaxAgent { max_depth: 7 },
            minimax::MinimaxAgent { max_depth: 7 },
        )
        .play_with_time_budget(time_budget_ms);
        print!(
            "{}",
            match winner {
                Player::Black => "B",
                Player::White => "W",
                Player::Tie => "T",
                _ => "?",
            }
        );
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        match winner {
            Player::Black => black_wins += 1,
            Player::White => white_wins += 1,
            Player::Tie => ties += 1,
            _ => {}
        }
    }
    println!();
    println!("Black wins: {black_wins}, White wins: {white_wins}, Ties: {ties}");
}
