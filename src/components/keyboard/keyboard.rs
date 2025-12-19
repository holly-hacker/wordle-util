use leptos::prelude::*;

use super::{Key, Row};

#[component]
pub fn Keyboard() -> impl IntoView {
    let key_rows = ["qwertyuiop", "asdfghjkl", "zxcvbnm"];

    view! {
        <div class="keyboard">
            <For each=move || key_rows key=|x| *x let(row)>
                <Row>
                    <For each=move || row.chars() key=|x| *x let(letter)>
                        <Key letter=letter />
                    </For>
                </Row>
            </For>
        </div>
    }
}
