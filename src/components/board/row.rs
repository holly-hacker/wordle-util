use leptos::prelude::*;

use super::Tile;

#[component]
pub fn Row(row_index: usize) -> impl IntoView {
    view! {
        <div class="row">
            <For each=|| 0..5 key=|i| *i let(tile_index)>
                <Tile row_index=row_index tile_index=tile_index />
            </For>
        </div>
    }
}
