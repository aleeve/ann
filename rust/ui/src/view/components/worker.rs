use futures::lock::Mutex;
use leptos::*;
use gloo_worker::reactor::{reactor, ReactorScope};
use gloo_worker::Spawnable;
use futures::{sink::SinkExt, StreamExt};
use crate::logic::model::client;

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


pub fn get_worker(input: ReadSignal<u64>) -> ReadSignal<Option<u64>> {
    let(result, set_result) = create_signal(None::<u64>);

    // Wrap bridge in Arc<Mutex> as I'm stupid and can't get it right otherwise
    let bridge = SquaredOnDemand::spawner().spawn("worker.js");
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

    create_resource(
        || "did:plc:klugggc44dmpomjkuzyahzjd", 
        |did| async {
                let mess = get_actor(did).await;
                gloo_console::log!(format!("{:?}", mess));
            }
    );
    result
}
    

#[component]
pub fn Worker() -> impl IntoView {    
    let(input, set_input) = create_signal(2);
    let result = get_worker(input);
    view! {<p> {move || result.get()}</p>}
}
