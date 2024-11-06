
use leptos::*;
use leptos_router::*;
use gloo_console::log;

use crate::logic::storage::Database;
use crate::view::components::navbar::NavBar;
use crate::view::components::worker::get_worker;
use crate::view::pages::{Home, Label};

#[component]
pub fn App() -> impl IntoView {
    // Setup worker
    let (input, set_input) = create_signal(2);
    let result = get_worker(input);
    provide_context(set_input);
    provide_context(result);

    // Provide db context
    let (stores, set_stores) = create_signal(vec!("models".to_string(), "actors".to_string()));
    let db = create_local_resource(
        move || stores.get(), 
        move |stores| async { 
            let db = Database::new(stores).await.unwrap();
            provide_context(db);
        }
    );

    view! {
        <div id="root">
            <Router>
                <NavBar/>
                <main>
                    <Routes>
                        <Route
                            path=""
                            view= move || {
                                view! {
                                    <Suspense fallback=move || view! { <div>"Loading..."</div> }>
                                    {move || {
                                            db.get();
                                            view! { <div><Home/> </div>}
                                        }}
                                    </Suspense>
                                }

                            }

                        />
                        <Route
                            path="settings"
                            view=Label
                        />
                        <Route
                            path="studio"
                            view=Label
                        />
                    </Routes>
                </main>
            </Router>
        </div>

    }
}
