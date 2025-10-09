use chrono::NaiveDate;
use dioxus::prelude::*;
use document::{Meta, Title};

use crate::web::{
    components::{guides::sidebar::GuideSidebar, Page, Section},
    model::{
        breadcrumb::Breadcrumb,
        guide::{GuideAuthor, GuideMeta, GuideOutline},
    },
    util::breadcrumb::path_to_breadcrumbs,
};

/// Formats the content of the guide and the author information
#[component]
pub fn Guide(
    meta: GuideMeta<'static>,
    outline: Vec<GuideOutline<'static>>,
    children: Element,
) -> Element {
    let path = router().full_route_string();

    let breadcrumbs: Vec<Breadcrumb> = path_to_breadcrumbs(path);

    let date = NaiveDate::parse_from_str(meta.date, "%Y-%m-%d")?;
    let formatted_date = date.format("%b %-d, %Y").to_string();

    rsx! {
        Title { "{meta.title} | Autumn Guides" }
        Meta {
            name: "description",
            content: meta.description
        }
        Page {
            Section {
                class: "flex min-h-screen",
                GuideSidebar {  class: "w-1/5" }
                div { class: "w-3/5 px-8",
                    div { class: "flex flex-col pb-4",
                        div { class: "breadcrumbs text-sm pb-4",
                            ul {
                                for (key, crumb) in breadcrumbs.iter().enumerate() {
                                    li { key: "{key}",
                                        a {
                                            href: "{crumb.href}", "{crumb.name}"
                                        }
                                    }
                                }
                            }
                        }
                        h1 { class: "font-bold text-xl xl:text-2xl", {meta.title} }
                        p { {formatted_date} }
                    }
                    article { class: "guide-content",
                        {children}
                    }
                }
                div { class: "w-1/5",
                    div { class: "sticky top-20 z-10 flex flex-col gap-4",
                        GuideAuthorSegment { author: meta.author, }
                        GuideOutlineSegment { outline: outline }
                    }
                }
            }
        }
    }
}

#[component]
pub fn GuideAuthorSegment(author: GuideAuthor<'static>) -> Element {
    rsx!(
        div { class: "flex flex-col gap-4 w-full pb-4 border-b border-base-200",
            p {
                class: "font-bold",
                "Posted by"
            }
            a { class: "hover:invert-[0.1]", href: "https://zkillboard.com/character/{author.character_id}/",
                div { class: "flex gap-4 items-center",
                    div { class: "avatar",
                        div { class: "w-16 rounded-full",
                            img { src: "https://images.evetech.net/characters/{author.character_id}/portrait?size=64" }
                        }
                    }
                    div {
                        p { {author.character_name} }
                        p { {author.title} }
                    }
                }
            }
        }
    )
}

#[component]
pub fn GuideOutlineSegment(outline: Vec<GuideOutline<'static>>) -> Element {
    rsx!(
        div { class: "flex flex-col gap-2 w-full",
            p {
                class: "font-bold",
                "On this page"
            }
            ul { class: "flex flex-col gap-1",
                for (key, entry) in outline.iter().enumerate() {
                    li { key: "{key}",
                        a { href: "#{entry.id}", class: "hover:text-primary",
                            p { {entry.title} }
                        }
                    }
                }
            }
        }
    )
}
