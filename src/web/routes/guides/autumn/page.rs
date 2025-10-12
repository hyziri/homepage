use dioxus::prelude::*;

use crate::web::{
    components::guides::guide::Guide, constant::guide::author::AUTHOR_HYZIRI,
    model::guide::GuideMeta, Route,
};

static AUTUMN_GUIDE_META: GuideMeta = GuideMeta {
    route: &Route::AutumnGuide {},
    title: "Autumn Guides",
    description: "Guides related to the Autumn experience",
    date: "2025-10-10",
    author: AUTHOR_HYZIRI,
};

#[component]
pub fn AutumnGuide() -> Element {
    let guide_outline = vec![];

    rsx! {
        Guide {
            meta: AUTUMN_GUIDE_META,
            outline: guide_outline,
        }
    }
}
