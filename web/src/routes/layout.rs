use dioxus::prelude::*;

use crate::components::{Footer, Header};
use crate::router::Route;

#[component]
pub fn Layout() -> Element {
    rsx! {
        Header {}
        Outlet::<Route> {}
        Footer {}
    }
}
