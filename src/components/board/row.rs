use leptos::prelude::*;

use super::Tile;

#[component]
pub fn Row() -> impl IntoView {
    view! {
        <div class="row">
            <Tile />
            <Tile />
            <Tile />
            <Tile />
            <Tile />
        </div>
    }
}
