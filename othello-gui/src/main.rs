use othello::mcts::Mcts;
use othello::othello::{Cell, Game, Play, Player};
use rand::seq::IndexedRandom;
use sycamore::prelude::*;
use sycamore::web::wasm_bindgen::prelude::*;
use web_sys::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Agent {
    Human,
    Computer(u32),
    Random,
}

fn get_move_for_agent(agent: Agent, game: Game) -> Option<Play> {
    match agent {
        Agent::Human => None,
        Agent::Computer(iterations_budget) => {
            let mut mcts_agent = Mcts::new(game);
            let search_res = mcts_agent.run_search_iterations_budget(iterations_budget);
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
    /// The actual game state.
    game: Signal<Game>,
    /// The game state displayed on the board.
    /// This is used to show game history.
    displayed_game: ReadSignal<Game>,
    /// The game state that is displayed when hovering over a cell.
    /// Shows the game state after a potential play is made.
    ghost_game: ReadSignal<Option<Game>>,
    /// The cell that is currently being hovered over.
    hovered_cell: Signal<Option<(u8, u8)>>,
    /// The history item that is currently being hovered over.
    hovered_history: Signal<Option<(Game, Play)>>,
    /// The history of the game, including the game state and the play that was made.
    /// The game state should be the state of the game _before_ the play was made.
    game_history: Signal<Vec<(Game, Play)>>,
    player_1: Signal<Agent>,
    player_2: Signal<Agent>,
}

#[component]
fn App() -> View {
    let hovered_cell = create_signal(None::<(u8, u8)>);
    let hovered_history = create_signal(None);

    let game = create_signal(Game::new());
    let displayed_game = create_memo(move || {
        if let Some((state, _)) = hovered_history.get() {
            state
        } else {
            game.get()
        }
    });
    let ghost_game = create_memo(move || {
        if let Some((row, col)) = hovered_cell.get() {
            let mut ghost_game = game.get();
            let play = Play::new(row, col);

            if ghost_game.is_valid_play(play) {
                ghost_game.make_play(play);
            }

            Some(ghost_game)
        } else if let Some((state, play)) = hovered_history.get() {
            let mut ghost_game = state;
            ghost_game.make_play(play);

            Some(ghost_game)
        } else {
            None
        }
    });

    let game_history = create_signal(Vec::new());

    let player_1 = create_signal(Agent::Human);
    let player_2 = create_signal(Agent::Human);

    let state = AppState {
        game,
        displayed_game,
        ghost_game,
        hovered_cell,
        hovered_history,
        game_history,
        player_1,
        player_2,
    };

    provide_context(state);

    // Run the current player's turn.
    // If the current player is a computer, it will make a move automatically.
    // If the current player is a human, it will wait for the human to make a move.
    fn _run_player_turn(
        state @ AppState {
            game,
            game_history,
            player_1,
            player_2,
            ..
        }: AppState,
    ) {
        let mut game_value = game.get();

        if game_value.game_state() != Player::InProgress {
            return;
        }

        let current_player = match game_value.player_to_move {
            Player::Black => player_1.get(),
            Player::White => player_2.get(),
            _ => return,
        };

        if let Some(play) = get_move_for_agent(current_player, game_value) {
            game_history.update(|history| history.push((game_value, play)));
            game_value.make_play(play);
            game.set(game_value);

            // Trigger the next player's turn after the current player has made a move.
            let closure = Closure::once_into_js(move || _run_player_turn(state));
            window().request_animation_frame(&closure.into()).unwrap();
        }
    }

    let run_player_turn = move || {
        let closure = Closure::once_into_js(move || _run_player_turn(state));
        window().request_animation_frame(&closure.into()).unwrap();
    };

    let onclick = move |row: u8, col: u8| {
        let mut game_value = game.get();
        let play = Play::new(row, col);

        if game_value.is_valid_play(play) {
            game_history.update(|history| history.push((game_value, play)));
            game_value.make_play(play);
            game.set(game_value);

            run_player_turn();
        }
    };

    view! {
        div(class="mx-auto my-4 max-w-prose") {
            h1(class="text-2xl font-bold") { "Othello" }

            h2(class="text-xl font-bold") { "Players" }
            div(class="flex flex-row justify-around my-2") {
                PlayerSelect(player=player_1, label="Player 1 (Black):")
                PlayerSelect(player=player_2, label="Player 2 (White):")
            }

            div(class="flex flex-row justify-around") {
                button(class="rounded px-4 py-2 bg-slate-900 text-white hover:bg-slate-800 transition-colors", on:click=move |_| {
                    game.set(Game::new());
                    game_history.set(Vec::new());
                    run_player_turn();
                }) { "New Game" }
            }

            div(class="flex flex-row justify-around my-4") {
                p(class="font-bold") {
                    (if game.get().game_state() != Player::InProgress {
                        match game.get().game_state() {
                            Player::Black => view! { "Black wins!" },
                            Player::White => view! { "White wins!" },
                            Player::Tie => view! { "It's a tie!" },
                            _ => unreachable!(),
                        }
                    }  else {
                        view! {
                            (format!("{:?}'s turn", game.get().player_to_move))
                        }
                    })
                }
            }

            div(class="flex flex-row") {
                GameBoard(onclick=onclick)
                ul(class="w-30 h-128 overflow-y-auto py-2 px-4") {
                    Indexed(
                        list=move || game_history.get_clone().into_iter().rev().collect::<Vec<_>>(),
                        view=move |(state, play)| {
                            view! {
                                li(
                                    class="cursor-default even:bg-gray-100 hover:even:bg-gray-200 odd:bg-white hover:odd:bg-gray-50",
                                    on:mouseover=move |_| {
                                        hovered_history.set(Some((state, play)));
                                    },
                                    on:mouseout=move |_| {
                                        hovered_history.set(None);
                                    }
                                ) {
                                    (if state.player_to_move == Player::Black {
                                        "B: "
                                    } else {
                                        "W: "
                                    })
                                    (format!("{play}"))
                                }
                            }
                        }
                    )
                }
            }

            div(class="text-sm text-gray-500 mt-10") {
                p {
                    "The computer uses the "
                    a(href="https://en.wikipedia.org/wiki/Monte_Carlo_tree_search", target="_blank", class="text-blue-500 hover:underline") {
                        "Monte Carlo Tree Search"
                    }
                    " (MCTS) algorithm to evaluate the best move. "
                    "The UI is implemented using the "
                    a(href="https://sycamore.dev", target="_blank", class="text-blue-500 hover:underline") {
                        "Sycamore"
                    }
                    " UI library in Rust and WebAssembly."
                }
                p {
                    "Find the source code at "
                    a(href="https://github.com/lukechu10/othello", target="_blank", class="text-blue-500 hover:underline") {
                        "github.com/lukechu10/othello"
                    }
                    "."
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
                    "Computer (Easy)" => Agent::Computer(500),
                    "Computer (Medium)" => Agent::Computer(1000),
                    "Computer (Hard)" => Agent::Computer(10000),
                    "Computer (Random)" => Agent::Random,
                    _ => Agent::Human,
                });
            }) {
                option(value="Human", selected=true) { "Human" }
                option(value="Computer (Easy)") { "Computer (Easy)" }
                option(value="Computer (Medium)") { "Computer (Medium)" }
                option(value="Computer (Hard)") { "Computer (Hard)" }
                option(value="Computer (Random)") { "Computer (Random)" }
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
        displayed_game,
        ghost_game,
        hovered_cell,
        ..
    } = use_context::<AppState>();

    let play = Play::new(row, col);

    let cell_state = move || displayed_game.get().cell_state(row, col);
    let is_valid_play = move || displayed_game.get().is_valid_play(play);

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

    let disc_class = move || match (cell_state(), ghost_cell_state()) {
        (Cell::Black, Some(Cell::White)) => "bg-red-950",
        (Cell::White, Some(Cell::Black)) => "bg-red-200",
        (Cell::Black, _) => "bg-slate-800",
        (Cell::White, _) => "bg-slate-100",
        (Cell::Empty, Some(Cell::Black)) => "bg-slate-800 opacity-30",
        (Cell::Empty, Some(Cell::White)) => "bg-slate-100 opacity-30",
        _ => "",
    };

    view! {
        button(
            class=format!("w-16 h-16 {} border-2 border-green-600 {} transition-colors", cell_color(), rounded()),
            on:click=onclick,
            on:mouseover=onmouseover,
            on:mouseout=onmouseout,
            disabled=cell_state() != Cell::Empty
        ) {
            div(class=format!("w-10 h-10 rounded-full {} m-3 inline-block transition-colors", disc_class())) {}
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(App);
}
