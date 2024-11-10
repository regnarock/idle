mod app;
mod components;
mod game;
mod storage;
use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Starting application...");
    yew::Renderer::<App>::new().render();
}
