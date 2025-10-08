use dioxus::prelude::*;

use crate::web::{
    components::guides::{author::AUTHOR_HYZIRI, layout::Guide},
    model::guide::GuideMeta,
};

pub static JOINING_AUTUMN_GUIDE_META: GuideMeta<'static> = GuideMeta {
    title: "Joining Autumn",
    description: "Guide on how to join The Order of Autumn",
    date: "2025-10-02",
    author: AUTHOR_HYZIRI,
};

#[component]
pub fn JoiningAutumnGuide() -> Element {
    rsx! {
        Guide {
            meta: JOINING_AUTUMN_GUIDE_META,
            h1 {
                "How to join Autumn"
            }
        }
    }
}
