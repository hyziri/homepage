use dioxus::prelude::*;

use crate::web::{
    components::{guides::autumn::sidebar::AutumnGuideSidebar, Page, Section},
    constant::guide::ACTIVE_AUTUMN_GUIDE_SUBCATEGORY,
    model::guide::AutumnGuideSubcategory,
    Route,
};

pub fn AutumnGuideLayout() -> Element {
    let path = router().full_route_string();

    if path.contains("/guides/autumn/highsec") {
        *ACTIVE_AUTUMN_GUIDE_SUBCATEGORY.write() = AutumnGuideSubcategory::HIGHSEC
    }

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
