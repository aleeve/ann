use crate::view::components::storage::StoreData;
use leptos::*;

#[component]
pub fn Label() -> impl IntoView {
    let (a, _) = create_signal("models".to_string());

    let result =
        use_context::<ReadSignal<Option<Vec<String>>>>().expect("This is set at the app start");
    let set_input =
        use_context::<WriteSignal<Vec<String>>>().expect("This is set at the app start");

    view! {
        <img src="logo.webp" alt="Logo" width="150" height="150"></img>
        <div class="card" >LABEL</div>

        <input
         type="string"
         on:input=move |ev| {
           gloo::console::log!("här");
           set_input.set(vec!(event_target_value(&ev).parse().unwrap_or(0).to_string()));
         }
         value={"hej"}
        />

        <div class="card has-background-primary">
            <StoreData key=a/>
        </div>
    }
}
