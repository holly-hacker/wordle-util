use leptos::prelude::*;

#[component]
pub fn Row(children: Children) -> impl IntoView {
    view! { <div class="row">{children()}</div> }
}
