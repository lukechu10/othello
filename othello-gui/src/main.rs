use sycamore::prelude::*;

#[component]
fn App() -> View {
    view! {
        div {
            h1 { "Othello Game" }
            p { "Welcome to the Othello game!" }
        }

    GameBoard()
    }
}

#[component]
fn GameBoard() -> View {
    view! {}
}

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(App);
}
