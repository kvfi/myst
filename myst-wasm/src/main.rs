mod app;
mod data;

use app::App;
use log::info;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("Hello");
    yew::Renderer::<App>::new().render();
}
