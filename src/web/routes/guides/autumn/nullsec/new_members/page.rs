use dioxus::prelude::*;

use crate::web::{
    components::guides::guide::Guide, constant::guide::author::AUTHOR_HYZIRI,
    model::guide::GuideMeta, Route,
};

pub static AUTUMN_NULLSEC_NEW_MEMBERS_GUIDE_META: GuideMeta = GuideMeta {
    route: &Route::AutumnNullsecNewMembersGuide {},
    name: "New Members",
    description: "Guides for new members of Autumn in nullsec",
    title: "Nullsec New Member Guides",
    date: "2025-10-10",
    author: AUTHOR_HYZIRI,
};

#[component]
pub fn AutumnNullsecNewMembersGuide() -> Element {
    let guide_outline = vec![];

    rsx! {
        Guide {
            meta: AUTUMN_NULLSEC_NEW_MEMBERS_GUIDE_META,
            outline: guide_outline,
        }
    }
}
