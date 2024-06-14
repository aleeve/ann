use crate::components::network::Connection;
use leptos::*;
use leptos_use::{use_websocket, UseWebsocketReturn};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Connection/>
        <div>{"HEj"}</div>
    }
}
