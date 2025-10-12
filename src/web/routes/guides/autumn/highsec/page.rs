use dioxus::prelude::*;

use crate::web::{
    components::guides::{author::AUTHOR_HYZIRI, guide::Guide},
    model::guide::GuideMeta,
    Route,
};

static AUTUMN_HIGHSEC_GUIDE_META: GuideMeta = GuideMeta {
    route: &Route::AutumnNullsecGuide {},
    title: "Autumn Highsec Guides",
    description: "Guides related to the Autumn highsec experience",
    date: "2025-10-10",
    author: AUTHOR_HYZIRI,
};

#[component]
pub fn AutumnHighsecGuide() -> Element {
    let guide_outline = vec![];

    rsx! {
        Guide {
            meta: AUTUMN_HIGHSEC_GUIDE_META,
            outline: guide_outline,
        }
    }
}
