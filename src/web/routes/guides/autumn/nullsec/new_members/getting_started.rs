use dioxus::prelude::*;

use crate::web::{
    components::guides::guide::Guide, constant::guide::author::AUTHOR_HYZIRI,
    model::guide::GuideMeta, Route,
};

pub static AUTUMN_NULLSEC_GETTING_STARTED_GUIDE_META: GuideMeta = GuideMeta {
    route: &Route::AutumnNullsecGettingStartedGuide {},
    title: "Getting Started",
    description: "Getting started in nullsec with Autumn",
    date: "2025-10-10",
    author: AUTHOR_HYZIRI,
};

#[component]
pub fn AutumnNullsecGettingStartedGuide() -> Element {
    let guide_outline = vec![];

    rsx! {
        Guide {
            meta: AUTUMN_NULLSEC_GETTING_STARTED_GUIDE_META,
            outline: guide_outline,
        }
    }
}
