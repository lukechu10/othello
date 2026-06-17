use crate::othello::{Game, Play, Player};

pub mod mcts;
pub mod mcts_eval;
pub mod mcts_mr;
pub mod minimax;
pub mod random;

pub trait Agent {
    /// Returns the best move for the current player given the current game state.
    ///
    /// This method should always complete in some reasonable finite amount of time.
    fn best_move(&self, game: Game) -> Play {
        self.best_move_with_time_budget(game, u64::MAX)
    }

    /// Returns the best move for the current player given the current game state and a time budget in milliseconds.
    ///
    /// This method should always complete in less than `time_budget_ms` milliseconds.
    /// It can run for less time if it finds a good enough move before the time budget is exhausted.
    fn best_move_with_time_budget(&self, game: Game, time_budget_ms: u64) -> Play;
}

pub struct Matchup {
    pub agent_black: Box<dyn Agent>,
    pub agent_white: Box<dyn Agent>,
}

impl Matchup {
    pub fn new(agent_black: impl Agent + 'static, agent_white: impl Agent + 'static) -> Self {
        Self {
            agent_black: Box::new(agent_black),
            agent_white: Box::new(agent_white),
        }
    }

    /// Plays a game between the two agents and returns the winner.
    pub fn play(&mut self) -> Player {
        let mut game = Game::new();

        while game.game_state() == Player::InProgress {
            let play = if game.player_to_move == Player::Black {
                self.agent_black.best_move(game)
            } else {
                self.agent_white.best_move(game)
            };
            if !game.is_valid_play(play) {
                panic!(
                    "Invalid play {:?} by player {:?}",
                    play, game.player_to_move
                );
            }
            game.make_play(play);
        }

        game.game_state()
    }

    pub fn play_with_time_budget(&mut self, time_budget_ms: u64) -> Player {
        let mut game = Game::new();

        while game.game_state() == Player::InProgress {
            let play = if game.player_to_move == Player::Black {
                self.agent_black
                    .best_move_with_time_budget(game, time_budget_ms)
            } else {
                self.agent_white
                    .best_move_with_time_budget(game, time_budget_ms)
            };
            game.make_play(play);
        }

        game.game_state()
    }
}
