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

/// Context for the application state.
#[derive(Clone, Copy)]
struct AppState {
    game: Signal<Game>,
    player_1: Signal<Agent>,
    player_2: Signal<Agent>,
    ghost_game: ReadSignal<Option<Game>>,
    hovered_cell: Signal<Option<(u8, u8)>>,
}

#[component]
fn App() -> View {
    let game = create_signal(Game::new());

    let hovered_cell = create_signal(None::<(u8, u8)>);
    let ghost_game = create_memo(move || {
        if let Some((row, col)) = hovered_cell.get() {
            let mut ghost_game = game.get();
            let play = Play::new(row, col);

            if ghost_game.is_valid_play(play) {
                ghost_game.make_play(play);
            }

            Some(ghost_game)
        } else {
            None
        }
    });

    let player_1 = create_signal(Agent::Human);
    let player_2 = create_signal(Agent::Human);

    // Run the current player's turn.
    // If the current player is a computer, it will make a move automatically.
    // If the current player is a human, it will wait for the human to make a move.
    fn _run_player_turn(game: Signal<Game>, player_1: Signal<Agent>, player_2: Signal<Agent>) {
        let mut game_value = game.get();

        if game_value.game_state() != Player::InProgress
            || game.get().generate_plays() == vec![Play(64)]
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

    provide_context(AppState {
        game,
        player_1,
        player_2,
        ghost_game,
        hovered_cell,
    });

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

            GameBoard(onclick=onclick)

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
fn GameBoard(onclick: impl Fn(u8, u8) + Copy + 'static) -> View {
    view! {
        div(class="flex flex-row justify-around") {
            div {
                ((0..8).map(move |row| {
                    view! {
                        div(class="row h-16") {
                            ((0..8).map(move |col| {
                                view! {
                                    Cell(row=row, col=col,onclick=onclick)
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
fn Cell(row: u8, col: u8, onclick: impl Fn(u8, u8) + 'static) -> View {
    let AppState {
        game,
        ghost_game,
        hovered_cell,
        ..
    } = use_context::<AppState>();

    let play = Play::new(row, col);

    let cell_state = move || game.get().cell_state(row, col);
    let is_valid_play = move || game.get().is_valid_play(play);

    let ghost_cell_state = move || {
        ghost_game
            .get()
            .map(|ghost_game| ghost_game.cell_state(row, col))
    };

    let onclick = move |_| {
        if cell_state() == Cell::Empty {
            onclick(row, col);
        }
    };

    let onmouseover = move |_| {
        hovered_cell.set(Some((row, col)));
    };
    let onmouseout = move |_| {
        hovered_cell.set(None);
    };

    let cell_color = move || {
        if is_valid_play() {
            "bg-green-600"
        } else {
            "bg-green-700"
        }
    };

    let rounded = move || {
        if row == 0 && col == 0 {
            "rounded-tl-lg"
        } else if row == 0 && col == 7 {
            "rounded-tr-lg"
        } else if row == 7 && col == 0 {
            "rounded-bl-lg"
        } else if row == 7 && col == 7 {
            "rounded-br-lg"
        } else {
            ""
        }
    };

    view! {
        button(
            class=format!("w-16 h-16 {} border border-green-900 {}", cell_color(), rounded()),
            on:click=onclick,
            on:mouseover=onmouseover,
            on:mouseout=onmouseout,
            disabled=cell_state() != Cell::Empty
        ) {
            (match (cell_state(), ghost_cell_state()) {
                (Cell::Black, Some(Cell::White)) => view! {
                    div(class="w-10 h-10 rounded-full bg-red-950 m-3 inline-block") {}
                },
                (Cell::White, Some(Cell::Black)) => view! {
                    div(class="w-10 h-10 rounded-full bg-red-200 m-3 inline-block") {}
                },
                (Cell::Black, _) => view! {
                    div(class="w-10 h-10 rounded-full bg-slate-800 m-3 inline-block") {}
                },
                (Cell::White, _) => view! {
                    div(class="w-10 h-10 rounded-full bg-slate-100 m-3 inline-block") {}
                },
                (Cell::Empty, Some(Cell::Black)) => view! {
                    div(class="w-10 h-10 rounded-full bg-slate-800 opacity-30 m-3 inline-block") {}
                },
                (Cell::Empty, Some(Cell::White)) => view! {
                    div(class="w-10 h-10 rounded-full bg-slate-100 opacity-30 m-3 inline-block") {}
                },
                _ => view! {
                    div(class="w-10 h-10 m-3 inline-block") {}
                },
            })
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(App);
}
