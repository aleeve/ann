use futures::lock::Mutex;
use leptos::*;
use gloo_worker::reactor::{reactor, Reactor, ReactorBridge, ReactorScope, ReactorScoped};
use gloo_worker::Spawnable;
use futures::{sink::SinkExt, StreamExt};

use std::sync::Arc;

use crate::logic::at_client::{get_actor, get_follows};

#[reactor]
pub async fn SquaredOnDemand(mut scope: ReactorScope<u64, u64>) {
    while let Some(m) = scope.next().await {
        // Do the heavy work!
        let result = m.checked_pow(2).unwrap_or(0);
        if scope.send(result).await.is_err() {
            // Something went wrong
            break;
        }
    }
}

#[reactor]
pub async fn AtprotoReactor(mut scope: ReactorScope<Vec<String>, Vec<String>>) {
    while let Some(m) = scope.next().await {
        // Do the heavy work!
        let result = vec!("hej".to_string());
        let did = "did:plc:klugggc44dmpomjkuzyahzjd";
        let mess = get_actor(did).await;
        gloo_console::log!(format!("{:?}", mess));

        if scope.send(result).await.is_err() {
            // Something went wrong
            break;
        }
    }
}

pub fn get_worker<I,O,R>(
    input: ReadSignal<I>, 
    bridge: ReactorBridge<R>
) -> ReadSignal<Option<O>>
where
    I: Clone + Eq,
    R: Reactor<Scope=ReactorScope<I,O>>
{
    let(result, set_result) = create_signal(None::<O>);

    // Wrap bridge in Arc<Mutex> as I'm stupid and can't get it right otherwise
    let bridge = Arc::new(Mutex::new(bridge));

    create_resource( move || input.get(), 
        move |input|{
            let bridge = bridge.clone();
            async move {
                let mut b = bridge.lock().await;
                b.send_input(input);
                let r = b.next().await;
                set_result.set(r);
            }
        }
    );

    result
}

pub fn get_atproto_worker(input: ReadSignal<Vec<String>>) -> ReadSignal<Option<Vec<String>>> {
    let bridge = AtprotoReactor::spawner().spawn("worker.js");
    get_worker(input, bridge)
}

pub fn get_square_worker(input: ReadSignal<u64>) -> ReadSignal<Option<u64>> {
    let bridge = SquaredOnDemand::spawner().spawn("worker.js");
    get_worker(input, bridge)
}

    
