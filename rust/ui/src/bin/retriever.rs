use gloo::worker::Registrable;
use js_sys;
use ui::view::components::workers::AtprotoReactor;

fn main() {
    console_error_panic_hook::set_once();
    let r = js_sys::eval("importScripts(self.location.origin + '/tslibs.js')");
    match r {
        Ok(_) => {}
        Err(e) => gloo::console::log!(e),
    }
    AtprotoReactor::registrar().register();
}
