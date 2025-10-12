use dioxus::prelude::*;

use crate::web::{
    components::guides::guide::Guide, constant::guide::author::AUTHOR_HYZIRI,
    model::guide::GuideMeta, Route,
};

pub static AUTUMN_HIGHSEC_NEW_MEMBERS_GUIDE_META: GuideMeta = GuideMeta {
    route: &Route::AutumnHighsecNewMembersGuide {},
    title: "New Members",
    description: "Guides for new members of Autumn in highsec",
    date: "2025-10-10",
    author: AUTHOR_HYZIRI,
};

#[component]
pub fn AutumnHighsecNewMembersGuide() -> Element {
    let guide_outline = vec![];

    rsx! {
        Guide {
            meta: AUTUMN_HIGHSEC_NEW_MEMBERS_GUIDE_META,
            outline: guide_outline,
        }
    }
}
