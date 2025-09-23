#![allow(non_snake_case)]

mod app;
mod components;
mod constant;
mod model;
mod router;
mod routes;

use crate::app::App;

fn main() {
    dioxus::launch(App)
}
