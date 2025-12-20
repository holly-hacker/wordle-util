use leptos::prelude::*;

use crate::solver::PossibleSolutions;

use super::Tile;

#[component]
pub fn Row(row_index: usize) -> impl IntoView {
    let solution = use_context::<Memo<PossibleSolutions>>().unwrap();
    let list = move || solution.read()[row_index].clone();

    view! {
        <div class="row">
            <div class="annotation left">
                <div class="inner"></div>
            </div>

            <For each=|| 0..5 key=|i| *i let(tile_index)>
                <Tile row_index=row_index tile_index=tile_index />
            </For>

            <div class="annotation right">
                <div class="inner">
                    <ShowLet some=list let:value>
                        {format!("Count: {}", value.get_count())}
                        <ShowLet some=move || value.get_words() let(words)>
                            {(!words.is_empty()).then(|| format!(" ({})", words.join(", ")))}
                        </ShowLet>
                    </ShowLet>
                </div>
            </div>
        </div>
    }
}
