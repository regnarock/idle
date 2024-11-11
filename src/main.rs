mod app;
mod components;
mod game;
mod hooks;
mod storage;
mod utils;
mod predifined_states;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Starting application...");
    yew::Renderer::<App>::new().render();
}