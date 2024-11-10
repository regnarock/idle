mod app;
mod components;
mod game;
mod storage;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
