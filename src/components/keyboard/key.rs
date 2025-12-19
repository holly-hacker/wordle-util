use leptos::prelude::*;

#[component]
pub fn Key(letter: char) -> impl IntoView {
    view! {
        <div class="key">
            <span>{letter}</span>
        </div>
    }
}
