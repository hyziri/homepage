use dioxus::prelude::*;

use crate::web::{
    components::guides::{author::AUTHOR_HYZIRI, guide::Guide},
    model::guide::{GuideEntry, GuideMeta},
};

static AUTUMN_NULLSEC_GUIDE: GuideEntry = GuideEntry {
    meta: AUTUMN_NULLSEC_GUIDE_META,
    href: "/guides/autumn/nullsec",
};

static AUTUMN_NULLSEC_GUIDE_META: GuideMeta = GuideMeta {
    title: "Autumn Nullsec Guides",
    description: "Guides related to the Autumn nullsec experience",
    date: "2025-10-10",
    author: AUTHOR_HYZIRI,
};

#[component]
pub fn AutumnNullsecGuide() -> Element {
    let guide_outline = vec![];

    rsx! {
        Guide {
            meta: AUTUMN_NULLSEC_GUIDE_META,
            outline: guide_outline,
        }
    }
}
