mod board;
mod keyboard;

use leptos::prelude::*;

use board::Board;
use keyboard::Keyboard;

use crate::letters::EnteredLetters;

#[component]
pub fn App() -> impl IntoView {
    // signal for content
    let (letters, set_letters) = signal(EnteredLetters::default());

    provide_context(letters);
    provide_context(set_letters);

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
            <h1>"Wordle Checker"</h1>
            <p>"This is a utility to give insight into your wordle play."</p>
            <p>
                "Enter your guesses below and click the letters to change their type. "
                "You can right-click a letter to remove it."
            </p>
        </div>
    }
}
