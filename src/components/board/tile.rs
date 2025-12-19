use leptos::prelude::*;

use crate::letters::{EnteredLetters, LetterState};

#[component]
pub fn Tile(row_index: usize, tile_index: usize) -> impl IntoView {
    let letters = use_context::<ReadSignal<EnteredLetters>>().unwrap();
    let set_letters = use_context::<WriteSignal<EnteredLetters>>().unwrap();

    let letter = move || letters.read().get(row_index, tile_index);
    let is_row_complete = move || letters.read().is_row_complete(row_index);
    let get_state = move || is_row_complete().then(|| letter().unwrap().state);

    view! {
        <div
            class="tile"
            class:empty=move || letter().is_none()
            class:populated=move || letter().is_some() && !is_row_complete()
            class:incorrect=move || get_state() == Some(LetterState::Incorrect)
            class:present=move || get_state() == Some(LetterState::Present)
            class:correct=move || get_state() == Some(LetterState::Correct)

            on:click=move |_| {
                set_letters.write().cycle_letter(row_index, tile_index);
            }

            // on right click
            on:contextmenu=move |ev| {
                ev.prevent_default();
                set_letters.write().remove_letter(row_index, tile_index);
            }
        >
            <span>{move || letter().map(|a| a.value).unwrap_or(' ')}</span>
        </div>
    }
}
