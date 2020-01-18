use log::trace;
use particules_app::Model;
use yew::App;

fn main() {
    web_logger::init();
    trace!("Initializing yew...");
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
