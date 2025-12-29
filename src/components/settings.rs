use leptos::prelude::*;

use crate::components::Settings;

const SPOIL_THRESHOLDS: [usize; 6] = [0, 1, 2, 3, 5, 10];

#[component]
pub fn SettingsComponent() -> impl IntoView {
    let settings = use_context::<ReadSignal<Settings>>().unwrap();
    let set_settings = use_context::<WriteSignal<Settings>>().unwrap();

    view! {
        <div class="settings">
            <h3 class="title">"Show possible words if only X remain:"</h3>
            <ul>
                <For each=|| SPOIL_THRESHOLDS key=|i| *i let(num)>
                    <li>
                        <button
                            on:click=move |_| set_settings.update(|s| s.show_top_words = num)
                            class:active=move || settings.get().show_top_words == num
                        >
                            {if num == 0 { "Never".into() } else { num.to_string() }}
                        </button>
                    </li>
                </For>
            </ul>
        </div>
    }
}
