use leptos::prelude::*;

use super::Row;

#[component]
pub fn Board() -> impl IntoView {
    view! {
        <div class="board">
            <For each=|| 0..6 key=|i| *i let(row_idx)>
                <Row row_index=row_idx />
            </For>
        </div>
    }
}
