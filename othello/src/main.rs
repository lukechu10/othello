use othello::agents::{Matchup, mcts::*};
use othello::othello::*;

fn main() {
    let mut game = Game::new();

    let agent_black = MctsAgent {
        max_iterations: 300,
    };
    let agent_white = MctsAgent {
        max_iterations: 3000,
    };

    let winner = Matchup::new(agent_black, agent_white).play();
    println!("{:?} wins", winner);
}
