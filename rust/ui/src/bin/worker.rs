use components::network::SquaredOnDemand;

use gloo::worker::Registrable;

fn main() {
    console_error_panic_hook::set_once();
    SquaredOnDemand::registrar().register();
}
