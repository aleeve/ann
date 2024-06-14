use leptos::*;
use leptos_use::{use_websocket, UseWebsocketReturn};
use gloo_worker::reactor::{reactor, ReactorScope};
use gloo_worker::Spawnable;
use futures::{sink::SinkExt, StreamExt};

#[reactor]
async fn SquaredOnDemand(mut scope: ReactorScope<u64, u64>) {
    while let Some(m) = scope.next().await {
        if scope.send(m.pow(2)).await.is_err() {
            break;
        }
    }
}

#[component]
pub fn Connection() -> impl IntoView {
    let UseWebsocketReturn {
        ready_state,
        message,
        message_bytes,
        send,
        send_bytes,
        open,
        close,
        ..
    } = use_websocket("localhost:8000");
    send_bytes(b"hej".to_vec());
    let status = move || ready_state.get().to_string();
    view! {<p> {status()}</p>}
}

#[component]
pub fn Worker() -> impl IntoView {    
    let mut bridge = SquaredOnDemand::spawner().spawn("...");
    view! {<p> {"Hej"}</p>}
}


