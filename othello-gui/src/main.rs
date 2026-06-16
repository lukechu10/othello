use othello::mcts::Mcts;
use othello::othello::{Cell, Game, Play, Player};
use rand::seq::IndexedRandom;
use sycamore::prelude::*;
use sycamore::web::wasm_bindgen::prelude::*;
use web_sys::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Agent {
    Human,
    Computer(u128),
    Random,
}

fn get_move_for_agent(agent: Agent, game: Game) -> Option<Play> {
    match agent {
        Agent::Human => None,
        Agent::Computer(time_budget) => {
            let mut mcts_agent = Mcts::new(game);
            let search_res = mcts_agent.run_search(time_budget);
            web_sys::console::log_1(
                &format!("{} games simulated", search_res.search_iterations).into(),
            );

            Some(mcts_agent.best_play())
        }
        Agent::Random => {
            let plays = game.generate_plays();
            plays.choose(&mut rand::rng()).copied()
        }
    }
}

#[component]
fn App() -> View {
    let game = create_signal(Game::new());

    let player_1 = create_signal(Agent::Human);
    let player_2 = create_signal(Agent::Human);

    // Run the current player's turn.
    // If the current player is a computer, it will make a move automatically.
    // If the current player is a human, it will wait for the human to make a move.
    fn _run_player_turn(game: Signal<Game>, player_1: Signal<Agent>, player_2: Signal<Agent>) {
        let mut game_value = game.get();

        if game_value.game_state() != Player::InProgress
            || game.get().generate_plays() == vec![Play::new(8, 8)]
        {
            return;
        }

        let current_player = match game_value.player_to_move {
            Player::Black => player_1.get(),
            Player::White => player_2.get(),
            _ => return,
        };

        if let Some(play) = get_move_for_agent(current_player, game_value) {
            game_value.make_play(play);
            game.set(game_value);

            // Trigger the next player's turn after the current player has made a move.
            let closure = Closure::once_into_js(move || _run_player_turn(game, player_1, player_2));
            window().request_animation_frame(&closure.into()).unwrap();
        }
    }

    let run_player_turn = move || {
        let closure = Closure::once_into_js(move || _run_player_turn(game, player_1, player_2));
        window().request_animation_frame(&closure.into()).unwrap();
    };

    let onclick = move |row: u8, col: u8| {
        let mut game_value = game.get();
        let play = Play::new(row, col);

        if game_value.is_valid_play(play) {
            game_value.make_play(play);
            game.set(game_value);

            run_player_turn();
        }
    };

    view! {
        div(class="mx-auto max-w-prose") {
            h1(class="text-2xl font-bold") { "Othello" }

            h2(class="text-xl font-bold") { "Players" }
            div(class="flex flex-row justify-around my-2") {
                PlayerSelect(player=player_1, label="Player 1 (Black):")
                PlayerSelect(player=player_2, label="Player 2 (White):")
            }

            div(class="flex flex-row justify-around") {
                button(class="rounded px-4 py-2 bg-slate-900 text-white hover:bg-slate-800 transition-colors", on:click=move |_| {
                    game.set(Game::new());
                    run_player_turn();
                }) { "New Game" }
            }

            div(class="flex flex-row justify-around my-4") {
                p {
                    "Current Player: " (format!("{:?}", game.get().player_to_move))
                }
                p {
                    "Game State: " (format!("{:?}", game.get().game_state()))
                }
            }

            GameBoard(game=game, onclick=onclick)

            div(class="text-sm text-gray-500 mt-10") {
                p {
                    "Find the source code at "
                    a(href="https://github.com/lukechu10/othello", target="_blank", class="text-blue-500 hover:underline") {
                        "github.com/lukechu10/othello"
                    }
                    "."
                }
                p {
                    "The AI is implemented using Monte Carlo Tree Search (MCTS). "
                    "The UI is implemented using the "
                    a(href="https://sycamore.dev", target="_blank", class="text-blue-500 hover:underline") {
                        "Sycamore"
                    }
                    " UI library in Rust and WebAssembly."
                }
            }
        }
    }
}

#[component(inline_props)]
fn PlayerSelect(player: Signal<Agent>, label: &'static str) -> View {
    view! {
        div {
            label {
                (label)
                " "
            }
            select(class="border", on:change=move |e: Event| {
                let value = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>().value();
                player.set(match value.as_str() {
                    "Human" => Agent::Human,
                    "Computer (Easy)" => Agent::Computer(10),
                    "Computer (Medium)" => Agent::Computer(100),
                    "Computer (Hard)" => Agent::Computer(1000),
                    "Random" => Agent::Random,
                    _ => Agent::Human,
                });
            }) {
                option(value="Human", selected=true) { "Human" }
                option(value="Computer (Easy)") { "Computer (Easy)" }
                option(value="Computer (Medium)") { "Computer (Medium)" }
                option(value="Computer (Hard)") { "Computer (Hard)" }
                option(value="Random") { "Random" }
            }
        }
    }
}

#[component(inline_props)]
fn GameBoard(game: Signal<Game>, onclick: impl Fn(u8, u8) + Copy + 'static) -> View {
    view! {
        div(class="flex flex-row justify-around") {
            div {
                ((0..8).map(move |row| {
                    view! {
                        div(class="row h-12") {
                            ((0..8).map(move |col| {
                                view! {
                                    Cell(row=row, col=col, game=game, onclick=onclick)
                                }
                            }).collect::<Vec<View>>())
                        }
                    }
                }).collect::<Vec<View>>())
            }
        }
    }
}

#[component(inline_props)]
fn Cell(row: u8, col: u8, game: Signal<Game>, onclick: impl Fn(u8, u8) + 'static) -> View {
    let play = Play::new(row, col);

    let cell_state = move || game.get().cell_state(row, col);
    let is_valid_play = move || game.get().is_valid_play(play);

    let onclick = move |_| {
        if cell_state() == Cell::Empty {
            onclick(row, col);
        }
    };

    let class = move || match cell_state() {
        Cell::Empty => {
            if is_valid_play() {
                "bg-green-500"
            } else {
                "bg-green-700"
            }
        }
        Cell::Black => "bg-slate-600",
        Cell::White => "bg-gray-200",
    };

    view! {
        button(class=format!("w-12 h-12 border {}", class()), on:click=onclick, disabled=cell_state() != Cell::Empty) {}
    }
}

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(App);
}
