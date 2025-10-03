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
            Section { class: "flex flex-col gap-4",
                h2 { class: "text-2xl font-bold",
                    "Autumn Guides"
                }
                ul { class: "flex gap-4",
                    li {
                        GuideCard {
                            meta: JOINING_AUTUMN_GUIDE_META,
                            href: "/guides/joining-autumn"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn GuideCard(meta: GuideMeta<'static>, href: String) -> Element {
    rsx! {
        a { href: href,
            div { class: "card card-border bg-base-100 shadow w-full md:w-96",
                div { class: "card-body",
                    h2 { class: "card-title",
                        {meta.title}
                    }
                    p {
                        {meta.description}
                    }
                    div { class: "flex justify-between items-center pt-8",
                        p { {meta.date} }
                        div { class: "flex items-center self-end gap-1 text-center",
                            div { class: "avatar",
                                div { class: "w-8 rounded-full",
                                    img { src: "https://images.evetech.net/characters/{meta.author.character_id}/portrait?size=64" }
                                }
                            }
                            p {
                                {meta.author.character_name}
                            }
                        }
                    }
                }
            }
        }
    }
}
