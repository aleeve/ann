use crate::components::network::Worker;
use crate::components::storage::StoreData;
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (a, _) = create_signal("hej".to_string());
    view! {
        <StoreData key=a/>
        <Worker />
        <div>{"HEj"}</div>
    }
}
