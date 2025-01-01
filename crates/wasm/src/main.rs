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
    console_log::init_with_level(log::Level::Debug).expect("console_log");

    yew::Renderer::<App>::new().render();
}
