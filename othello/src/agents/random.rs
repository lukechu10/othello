use rand::seq::IndexedRandom;

use crate::{
    agents::Agent,
    othello::{Game, Play},
};

#[derive(Clone, Copy)]
pub struct RandomAgent;

impl Agent for RandomAgent {
    fn best_move_with_time_budget(&self, game: Game, _time_budget_ms: u64) -> Play {
        let plays = game.generate_plays();
        let mut rng = rand::rng();
        *plays.choose(&mut rng).unwrap()
    }
}
