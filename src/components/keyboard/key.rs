use leptos::prelude::*;

use crate::letters::{EnteredLetters, LetterState};

#[component]
pub fn Key(letter: char) -> impl IntoView {
    let letters = use_context::<ReadSignal<EnteredLetters>>().unwrap();
    let set_letters = use_context::<WriteSignal<EnteredLetters>>().unwrap();

    let letter_state = move || letters.get().get_letter_state(letter);

    view! {
        <div
            class="key"
            on:click=move |_| set_letters.write().push_letter(letter)
            class:incorrect=move || letter_state() == Some(LetterState::Incorrect)
            class:present=move || letter_state() == Some(LetterState::Present)
            class:correct=move || letter_state() == Some(LetterState::Correct)
        >
            <span>{letter}</span>
        </div>
    }
}
