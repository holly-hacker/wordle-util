#![allow(clippy::module_inception)]

mod components;
mod letters;

use leptos::prelude::*;
use log::Level;

fn main() {
    _ = console_log::init_with_level(Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(components::App)
}
