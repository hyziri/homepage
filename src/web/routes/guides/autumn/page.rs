use dioxus::prelude::*;

use crate::web::{
    components::guides::{author::AUTHOR_HYZIRI, guide::Guide},
    model::guide::{GuideEntry, GuideMeta},
};

static AUTUMN_GUIDE: GuideEntry = GuideEntry {
    meta: AUTUMN_GUIDE_META,
    href: "/guides/autumn",
    entries: &[],
};

static AUTUMN_GUIDE_META: GuideMeta = GuideMeta {
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
