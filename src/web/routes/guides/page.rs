use dioxus::prelude::*;
use document::{Meta, Title};

use crate::web::{
    components::guides::model::GuideMeta, routes::guides::joining_autumn::JOINING_AUTUMN_GUIDE_META,
};

#[component]
pub fn GuidesDirectory() -> Element {
    rsx! {
        Title {"Guides | Autumn"}
        Meta {
            name: "description",
            content: "Guides by The Order of Autumn"
        }
        section { class: "flex justify-center min-h-screen pt-[64px]",
            div { class: "max-w-[1440px] p-6 w-full h-full",
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
