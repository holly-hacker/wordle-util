mod board;
mod keyboard;

use leptos::prelude::*;

use board::Board;
use keyboard::Keyboard;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="container">
            <Introduction />
            <Board />
            <Keyboard />
        </div>
    }
}

#[component]
pub fn Introduction() -> impl IntoView {
    view! {
        <div class="introduction">
            <h1>"Wordle Util"</h1>
            <p>"This is a utility to give insight into your wordle play."</p>
            <p>"Enter your guesses below and click the letters to change their type."</p>
        </div>
    }
}
