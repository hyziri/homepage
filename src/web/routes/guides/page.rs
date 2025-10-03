use dioxus::prelude::*;
use document::{Meta, Title};

use crate::web::{
    components::{guides::model::GuideMeta, Page, Section},
    routes::guides::joining_autumn::JOINING_AUTUMN_GUIDE_META,
};

#[component]
pub fn GuidesDirectory() -> Element {
    rsx! {
        Title {"Guides | Autumn"}
        Meta {
            name: "description",
            content: "Guides by The Order of Autumn"
        }
        Page {
            Section {
                ul {
                    GuideCard {
                        meta: JOINING_AUTUMN_GUIDE_META,
                        href: "/guides/joining-autumn"
                    }
                }
            }
        }
    }
}

#[component]
pub fn GuideCard(meta: GuideMeta<'static>, href: &'static str) -> Element {
    rsx! {
        div {
            "This is a guide Entry"
        }
    }
}
