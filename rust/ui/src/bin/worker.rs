use gloo::worker::Registrable;
use ui::view::components::workers::SquaredOnDemand;

fn main() {
    console_error_panic_hook::set_once();
    SquaredOnDemand::registrar().register();
}
