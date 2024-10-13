use codee::binary::MsgpackSerdeCodec;

use leptos::*;
use leptos_use::{use_websocket, UseWebSocketReturn};

#[component]
pub fn Connection() -> impl IntoView {
    let (a, b) = create_signal(Vec::<u8>::default());
    let c = Callback::new(move |v: Vec<u8>| b.set(v));

    let UseWebSocketReturn {
        ready_state,
        message,
        send,
        ..
    } = use_websocket::<String, String, MsgpackSerdeCodec>("localhost:8000");

    send(&"hej".to_string());
    let status = move || ready_state.get().to_string();
    view! {
        <p> {
            c.call(a.get());
            status()}
        </p>
    }
}
