use dioxus::prelude::*;

use crate::web::{
    components::{guides::autumn::sidebar::AutumnGuideSidebar, Page, Section},
    model::guide::SelectedAutumnGuideCategory,
    Route,
};

pub fn AutumnGuideLayout() -> Element {
    let selected_category = use_signal(|| SelectedAutumnGuideCategory::NULLSEC);

    rsx! (
        Page {
            Section {
                class: "flex min-h-screen",
                AutumnGuideSidebar { class: "w-1/5", selected_category: selected_category }
                Outlet::<Route> {}
            }
        }
    )
}
