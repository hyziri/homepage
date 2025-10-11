use chrono::NaiveDate;
use dioxus::prelude::*;
use document::{Meta, Title};

use crate::web::{
    components::{
        guides::author::GuideAuthorSegment, guides::autumn::sidebar::AutumnGuideSidebar,
        guides::outline::GuideOutlineSegment, Page, Section,
    },
    model::{
        breadcrumb::Breadcrumb,
        guide::{GuideMeta, GuideOutline},
    },
    util::breadcrumb::path_to_breadcrumbs,
};

/// Formats the content of the guide and the author information
#[component]
pub fn Guide(meta: GuideMeta<'static>, outline: Vec<GuideOutline>, children: Element) -> Element {
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
                AutumnGuideSidebar {  class: "w-1/5" }
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
                        h1 { class: "font-bold text-2xl xl:text-4xl", {meta.title} }
                        p { {formatted_date} }
                    }
                    article { class: "guide-content prose",
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
