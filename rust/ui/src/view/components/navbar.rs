use leptos::*;
use leptos_router::*;

#[component]
pub fn NavBar() -> impl IntoView {
    let (burger_active, set_burger_active) = create_signal(false);

    view! {
        <nav class="navbar" role="navigation">

            <div class="navbar-brand">
                <A class="navbar-item" href="">"hacks"</A>
                <a role="button"
                   class={ move || format!("navbar-burger {}", if burger_active.get() {"is-active"} else {""})}
                   aria-label="menu"
                   aria-expanded="false"
                   data-target="navbar"
                   on:click= move |_| set_burger_active.set(!burger_active.get())
                >
                    <span aria-hidden="true"/><span aria-hidden="true"/><span aria-hidden="true"/>
                </a>
            </div>

            <div class={ move || format!("navbar-menu {}", if burger_active.get() {"is-active"} else {""})}
                 id="navbar"
            >
                <div class="navbar-start">
                    <A class="navbar-item" href=""> Home </A>
                    <A class="navbar-item" href="studio"> Studio </A>
                </div>
                <div class="navbar-end">
                    <A class="navbar-item" href="settings"> Settings </A>
                </div>
            </div>
        </nav>
    }
}
