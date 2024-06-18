mod app;
mod components;
mod model;

use console_error_panic_hook;
use leptos::*;
use std::panic;

use crate::app::App;

fn main() {
    // Add nice console stacktraces
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    // Mount the actual app
    mount_to_body(|| view! { <App/>})
}
