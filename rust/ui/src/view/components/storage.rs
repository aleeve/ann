
use crate::logic::storage::Database;
use leptos::*;
use serde_json::json;

#[component]
pub fn StoreData(
    #[prop(into)]
    key: ReadSignal<String>
) -> impl IntoView {
    let db = use_context::<Database>();
    let key = create_memo(move |_| key.get());

    let a = move || {
        let db = db.clone();
        match db {
            Some(db) => {
                create_resource(
                    move || key.get(), 
                    move |key : String|{ 
                        let db = db.clone();
                        async move {
                            let data = json!({"hash": 12, "value": 123});
                            db.add_data(key.as_str(), &data).await.expect("Error?");
                        }
                    });
            },
            None => {gloo_console::log!("Aha");},
        }
        view!{<h1>"My Data"</h1>}
    };

    a
}
