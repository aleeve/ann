use crate::view::components::storage::StoreData;
use crate::view::components::worker::Worker;
use leptos::*;

#[component]
pub fn Label() -> impl IntoView {
    let (a, _) = create_signal("eghed!".to_string());
    view! {
        <img src="logo.webp" alt="Logo" width="150" height="150"></img>
        <div class="card" >LABEL</div>
        <div class="card has-background-primary">
            <StoreData key=a/>
            // <Worker />
        </div>
    }
}
