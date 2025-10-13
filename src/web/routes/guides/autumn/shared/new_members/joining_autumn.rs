use dioxus::prelude::*;

use crate::web::{
    components::guides::guide::Guide, constant::guide::author::AUTHOR_HYZIRI,
    model::guide::GuideMeta, Route,
};

pub static AUTUMN_JOIN_GUIDE_META: GuideMeta = GuideMeta {
    route: &Route::AutumnJoinGuide {},
    name: "Joining Autumn",
    title: "How to Join Autumn",
    description:
        "How to join Autumn in highsec with Autumn Inc. or nullsec with The Order of Autumn",
    date: "2025-10-13",
    author: AUTHOR_HYZIRI,
};

#[component]
pub fn AutumnJoinGuide() -> Element {
    let guide_outline = vec![];

    rsx! {
        Guide {
            meta: AUTUMN_JOIN_GUIDE_META,
            outline: guide_outline,
        }
    }
}
