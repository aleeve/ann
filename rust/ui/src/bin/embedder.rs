use gloo::worker::Registrable;
use ui::view::components::workers::AtprotoReactor;

fn main() {
    console_error_panic_hook::set_once();
    AtprotoReactor::registrar().register();
}
