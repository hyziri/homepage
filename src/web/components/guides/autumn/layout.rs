use dioxus::prelude::*;

use crate::web::{
    components::{guides::autumn::sidebar::AutumnGuideSidebar, Page, Section},
    model::guide::AutumnGuideSubcategory,
    Route,
};

pub fn AutumnGuideLayout() -> Element {
    let path = router().full_route_string();

    let subcategory = use_signal(|| {
        if path.contains("/guides/autumn/highsec") {
            AutumnGuideSubcategory::HIGHSEC
        } else {
            AutumnGuideSubcategory::NULLSEC
        }
    });

    rsx! (
        Page {
            Section {
                class: "flex min-h-screen",
                AutumnGuideSidebar { class: "w-1/5", subcategory: subcategory }
                Outlet::<Route> {}
            }
        }
    )
}
