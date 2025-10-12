use dioxus::prelude::*;

use crate::web::{
    components::guides::guide::Guide, constant::guide::author::AUTHOR_HYZIRI,
    model::guide::GuideMeta, Route,
};

pub static AUTUMN_HIGHSEC_GETTING_STARTED_GUIDE_META: GuideMeta = GuideMeta {
    route: &Route::AutumnHighsecGettingStartedGuide {},
    title: "Getting Started",
    description: "Getting started in highsec with Autumn",
    date: "2025-10-10",
    author: AUTHOR_HYZIRI,
};

#[component]
pub fn AutumnHighsecGettingStartedGuide() -> Element {
    let guide_outline = vec![];

    rsx! {
        Guide {
            meta: AUTUMN_HIGHSEC_GETTING_STARTED_GUIDE_META,
            outline: guide_outline,
        }
    }
}
