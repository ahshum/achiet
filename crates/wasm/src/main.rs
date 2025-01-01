mod app;
mod auth;
mod bookmark;
mod context;
mod form;
mod hooks;
mod icons;
mod router;
mod tag;

use app::App;

fn main() {
    wasm_log::init(wasm_log::Config::default());

    yew::Renderer::<App>::new().render();
}
