mod game;
mod models;
mod view;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<view::App>::new().render();
}
