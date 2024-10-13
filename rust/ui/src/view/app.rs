use crate::view::components::navbar::NavBar;
use crate::view::components::worker::get_worker;
use crate::view::pages::{Home, Label};
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    let (input, set_input) = create_signal(2);
    let result = get_worker(input);
    provide_context(set_input);
    provide_context(result);

    view! {
        <div id="root">
            <Router>
                <NavBar/>
                <main>
                    <Routes>
                        <Route
                            path=""
                            view=Home
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
