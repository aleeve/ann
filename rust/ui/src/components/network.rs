use leptos::*;
use leptos_use::{use_websocket, UseWebsocketReturn};
use gloo_worker::reactor::{reactor, Reactor, ReactorScope};
use gloo_worker::Spawnable;
use futures::{sink::SinkExt, StreamExt};

#[reactor]
pub async fn SquaredOnDemand(mut scope: ReactorScope<u64, u64>) {
    while let Some(m) = scope.next().await {
        if scope.send(m.pow(2)).await.is_err() {
            break;
        }
    }
}

#[component]
pub fn Connection() -> impl IntoView {
    let (a,b) = create_signal(Vec::<u8>::default());
    let c = Callback::new(move |v: Vec::<u8>| b.set(v));

    let UseWebsocketReturn {
        ready_state,
        message_bytes,
        send_bytes,
        ..
    } = use_websocket("localhost:8000");
    send_bytes(b"hej".to_vec());
    let status = move || ready_state.get().to_string();
    view! {
        <p> {
            c.call(a.get());
            status()}
        </p>
    }
}

#[component]
pub fn Worker() -> impl IntoView {    
    let(input, set_input) = create_signal(2);
    let(result, set_result) = create_signal(None::<u64>);
    create_resource( move || input.get(), 
        move |i|{
            async move {
                let mut bridge = SquaredOnDemand::spawner().spawn("worker.js");
                bridge.send_input(i);
                while let r = bridge.next().await{
                    set_result.set(r);
                }
            }
        }
    );
    
    view! {<p> {move || result.get()}</p>}
}


