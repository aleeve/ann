use crate::logic::storage::hej;
use gloo_console::log;
use leptos::*;

#[component]
pub fn StoreData(key: ReadSignal<String>) -> impl IntoView {
    log!("Storing data");
    let a = create_resource(move || key.get(), hej);

    view! {
        <h1>"My Data"</h1>
        {move || match a.get() {
            None => view! { <p>"Loading..."</p> }.into_view(),
            Some(data) => view! { <div> {data.to_string()}</div> }.into_view()
        }}
    }
}
