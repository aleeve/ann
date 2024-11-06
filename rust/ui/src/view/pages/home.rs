use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
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
    }
}
