use crate::{
    agents::Agent,
    othello::{Game, Play, Player},
};

#[derive(Clone, Copy)]
pub struct MinimaxAgent {
    pub max_depth: u32,
}

impl Agent for MinimaxAgent {
    fn best_move(&mut self, game: Game) -> Play {
        minimax(game, self.max_depth, game.player_to_move == Player::Black).0
    }

    fn best_move_with_time_budget(&mut self, game: Game, _time_budget_ms: u64) -> Play {
        // For simplicity, we ignore the time budget in this implementation.
        self.best_move(game)
    }
}

fn evaluate_game(game: &Game) -> i32 {
    match game.game_state() {
        Player::Black => 64,
        Player::White => -64,
        Player::Tie => 0,
        _ => {
            let black_count = game.black_pieces.0.count_ones() as i32;
            let white_count = game.white_pieces.0.count_ones() as i32;
            black_count - white_count
        }
    }
}

fn minimax(game: Game, depth: u32, maximizing_player: bool) -> (Play, i32) {
    fn alphabeta(
        game: Game,
        depth: u32,
        mut alpha: i32,
        mut beta: i32,
        maximizing_player: bool,
    ) -> (Play, i32) {
        if depth == 0 || game.game_state() != Player::InProgress {
            let score = evaluate_game(&game);
            return (Play(0), score);
        }

        let plays = game.generate_plays();
        let mut best_play = Play(0);

        if maximizing_player {
            let mut best_score = i32::MIN;

            for play in plays {
                let mut next_game = game;
                next_game.make_play(play);

                let (_child_play, score) = alphabeta(next_game, depth - 1, alpha, beta, false);

                if score > best_score {
                    best_score = score;
                    best_play = play;
                }

                alpha = alpha.max(best_score);
                if beta <= alpha {
                    break; // beta cut-off
                }
            }

            (best_play, best_score)
        } else {
            let mut best_score = i32::MAX;

            for play in plays {
                let mut next_game = game;
                next_game.make_play(play);

                let (_child_play, score) = alphabeta(next_game, depth - 1, alpha, beta, true);

                if score < best_score {
                    best_score = score;
                    best_play = play;
                }

                beta = beta.min(best_score);
                if beta <= alpha {
                    break; // alpha cut-off
                }
            }

            (best_play, best_score)
        }
    }

    alphabeta(game, depth, i32::MIN, i32::MAX, maximizing_player)
}
