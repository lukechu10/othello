use rand::seq::IndexedRandom;

use crate::{
    agents::Agent,
    othello::{Game, Play, Player},
};

#[derive(Clone, Copy)]
pub struct MinimaxAgent {
    pub max_depth: u32,
}

impl Agent for MinimaxAgent {
    fn best_move(&self, game: Game) -> Play {
        // If we have less than 8 pieces on the board, just pick a random move.
        // This is so that we don't get the same games over and over again since minimax is
        // deterministic.
        if game.black_pieces.0.count_ones() + game.white_pieces.0.count_ones() < 8 {
            let plays = game.generate_plays();
            let mut rng = rand::rng();
            *plays.choose(&mut rng).unwrap()
        } else {
            minimax(game, self.max_depth).0
        }
    }

    fn best_move_with_time_budget(&self, game: Game, _time_budget_ms: u64) -> Play {
        // For simplicity, we ignore the time budget in this implementation.
        self.best_move(game)
    }
}

fn evaluate_game(game: &Game) -> i32 {
    match game.game_state() {
        Player::Black | Player::White => {
            let black_count = game.black_pieces.0.count_ones() as i32;
            let white_count = game.white_pieces.0.count_ones() as i32;
            1_000_000 * (black_count - white_count)
        }
        Player::Tie => 0,
        _ => {
            let black_count = game.black_pieces.0.count_ones() as i32;
            let white_count = game.white_pieces.0.count_ones() as i32;
            // Count corners and edges for better evaluation
            let corners = 0x8100000000000081u64; // Corners of the board
            let edges =
                0b01111110_10000001_10000001_10000001_10000001_10000001_10000001_01111110u64; // Edges of the board
            let black_corners = (game.black_pieces.0 & corners).count_ones() as i32;
            let white_corners = (game.white_pieces.0 & corners).count_ones() as i32;
            let black_edges = (game.black_pieces.0 & edges).count_ones() as i32;
            let white_edges = (game.white_pieces.0 & edges).count_ones() as i32;

            // black_count - white_count
            (black_count - white_count)
                + 10 * (black_corners - white_corners)
                + 2 * (black_edges - white_edges)
        }
    }
}

pub fn minimax(game: Game, depth: u32) -> (Play, i32) {
    /// Run the minimax algorithm with alpha-beta pruning.
    ///
    /// # Params
    /// - `game`: The current game state.
    /// - `depth`: The maximum depth to search.
    /// - `alpha`: The best score that the maximizer currently can guarantee at that level.
    /// - `beta`: The best score that the minimizer currently can guarantee at that level.
    /// - `plays_bufs`: A buffer to store the generated plays for each depth level. This prevents repeated allocations and improves performance. The buffer is indexed by depth, where `plays_bufs[depth]` contains the generated plays for that depth level.
    fn alphabeta(
        game: Game,
        depth: u32,
        mut alpha: i32,
        mut beta: i32,
        plays_bufs: &mut [Vec<Play>],
    ) -> (Play, i32) {
        if depth == 0 || game.game_state() != Player::InProgress {
            let score = evaluate_game(&game);
            return (Play(0), score);
        }

        let mut plays = std::mem::take(&mut plays_bufs[depth as usize]);
        assert_eq!(plays.capacity(), 32);
        plays.clear();
        game.generate_plays_in_buf(&mut plays);
        let mut best_play = Play(0);
        let mut best_score;

        if game.player_to_move == Player::Black {
            best_score = i32::MIN;

            for &play in &plays {
                let mut next_game = game;
                next_game.make_play(play);

                let (_child_play, score) = alphabeta(next_game, depth - 1, alpha, beta, plays_bufs);

                if score > best_score {
                    best_score = score;
                    best_play = play;
                }

                alpha = alpha.max(best_score);
                if beta <= alpha {
                    break; // beta cut-off
                }
            }
        } else {
            best_score = i32::MAX;

            for &play in &plays {
                let mut next_game = game;
                next_game.make_play(play);

                let (_child_play, score) = alphabeta(next_game, depth - 1, alpha, beta, plays_bufs);

                if score < best_score {
                    best_score = score;
                    best_play = play;
                }

                beta = beta.min(best_score);
                if beta <= alpha {
                    break; // alpha cut-off
                }
            }
        }
        // Replace the plays buffer for this depth level back so that it can be reused in future calls.
        plays_bufs[depth as usize] = plays;
        (best_play, best_score)
    }

    let mut plays_bufs: Vec<Vec<Play>> = (0..=depth).map(|_| Vec::with_capacity(32)).collect();

    alphabeta(game, depth, i32::MIN, i32::MAX, &mut plays_bufs)
}
