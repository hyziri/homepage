use chrono::NaiveDate;
use dioxus::prelude::*;
use document::{Meta, Title};

use crate::web::{
    components::{Page, Section},
    model::guide::{GuideCategory, GuideMeta},
    routes::guides::new_members::NEW_MEMBER_GUIDE_CATEGORY,
};

pub static GUIDE_CATEGORIES: [GuideCategory; 1] = [NEW_MEMBER_GUIDE_CATEGORY];

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
                h1 { class: "text-4xl font-bold",
                    "Autumn Guides"
                }
                ul { class: "flex flex-col gap-4",
                    for (key, category) in GUIDE_CATEGORIES.iter().enumerate() {
                        li { key: "{key}",
                            GuideCategorySection { category: *category }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn GuideCategorySection(category: GuideCategory<'static>) -> Element {
    rsx! (
        div { class: "flex flex-col gap-4",
            a { href: "{category.page.href}", class: "hover:text-primary",
                h2 { class: "font-bold text-2xl", "{category.page.meta.title}" }
            }
            ul { class: "flex gap-4",
                for (key, guide) in category.entries.iter().enumerate() {
                    li { key: "{key}",
                        GuideCard {
                            meta: guide.meta,
                            href: guide.href
                        }
                    }
                }
            }
        }
    )
}

#[component]
pub fn GuideCard(meta: GuideMeta<'static>, href: String) -> Element {
    let date = NaiveDate::parse_from_str(meta.date, "%Y-%m-%d")?;
    let formatted_date = date.format("%b %-d, %Y").to_string();

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
                        p { {formatted_date} }
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
