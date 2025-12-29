mod board;
mod keyboard;
mod settings;

use board::Board;
use keyboard::Keyboard;
use leptos::prelude::*;
use log::debug;
use settings::SettingsComponent;

use crate::letters::EnteredLetters;

#[derive(Clone, Default)]
pub struct Settings {
    pub show_top_words: usize,
}

#[component]
pub fn App() -> impl IntoView {
    // signal for content
    let (letters, set_letters) = signal(EnteredLetters::default());
    let (settings, set_settings) = signal(Settings::default());

    provide_context(letters);
    provide_context(set_letters);
    provide_context(settings);
    provide_context(set_settings);

    let valid_word_count = Memo::new(move |_| {
        let letters = letters.read();
        let valid_words = crate::solver::get_valid_words(&letters);
        debug!("Calculated valid word results: {valid_words:?}");
        valid_words
    });
    provide_context(valid_word_count);

    let onkeypress = window_event_listener(leptos::ev::keydown, move |ev| {
        let key = ev.key();

        if key == "Backspace" || key == "Delete" {
            set_letters.write().remove_last_letter();
            ev.prevent_default();
            return;
        }

        let mut chars = key.chars();
        if let (Some(char), None) = (chars.next(), chars.next())
            && char.to_ascii_lowercase().is_ascii_lowercase()
        {
            ev.prevent_default();
            set_letters.write().push_letter(char);
        }
    });

    on_cleanup(move || onkeypress.remove());

    view! {
        <div class="container">
            <Introduction />
            <Board />
            <Keyboard />
            <SettingsComponent />
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
