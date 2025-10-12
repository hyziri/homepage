use dioxus::prelude::*;

use crate::web::{
    components::{guides::autumn::sidebar::AutumnGuideSidebar, Page, Section},
    Route,
};

pub fn AutumnGuideLayout() -> Element {
    rsx! (
        Page {
            Section {
                class: "flex min-h-screen",
                AutumnGuideSidebar { class: "w-1/5" }
                Outlet::<Route> {}
            }
        }
    )
}
