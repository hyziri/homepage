use dioxus::prelude::*;

use crate::web::{
    components::guides::guide::Guide, constant::guide::author::AUTHOR_HYZIRI,
    model::guide::GuideMeta, Route,
};

static AUTUMN_HIGHSEC_GUIDE_META: GuideMeta = GuideMeta {
    route: &Route::AutumnHighsecGuide {},
    name: "Highsec Guides",
    description: "Guides related to the Autumn highsec experience",
    title: "Autumn Highsec Guides",
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
