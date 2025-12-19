use leptos::prelude::*;

#[component]
pub fn Tile() -> impl IntoView {
    view! {
        <div class="tile populated">
            <span>"X"</span>
        </div>
    }
}
