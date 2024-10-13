use crate::view::components::storage::StoreData;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    let (a, _) = create_signal("eghed!".to_string());

    let result = use_context::<ReadSignal<Option<u64>>>().expect("This is set at the app start");
    let set_input = use_context::<WriteSignal<u64>>().expect("This is set at the app start");

    view! {
        <div class="card" >HOME</div>
        <input type="int"
            on:input=move |ev| {
             set_input.set(event_target_value(&ev).parse().unwrap_or(0));
            }
        />
        <div>{move || result.get()}</div>
        <div class="card has-background-primary">
            <StoreData key=a/>
        </div>
    }
}
