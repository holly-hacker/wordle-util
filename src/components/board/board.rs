use leptos::prelude::*;

use super::Row;

#[component]
pub fn Board() -> impl IntoView {
    view! {
        <div class="board">
            <Row />
            <Row />
            <Row />
            <Row />
            <Row />
            <Row />
        </div>
    }
}
