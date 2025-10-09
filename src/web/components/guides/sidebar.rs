use dioxus::prelude::*;

use crate::web::{model::guide::GuideCategory, routes::guides::page::GUIDE_CATEGORIES};

#[component]
pub fn GuideSidebar(class: Option<&'static str>) -> Element {
    let class: &str = if let Some(class) = class { class } else { "" };

    rsx!(
        div { class: "{class}",
            div { class: "sticky top-20 z-10 flex flex-col gap-1",
                a { href: "/guides", class: "hover:text-primary",
                    h2 { class: "font-bold text-xl", "Guides" }
                }
                ul {
                    for (key, category) in GUIDE_CATEGORIES.iter().enumerate() {
                        li { key: "{key}",
                            GuideSidebarCategory { category: *category }
                        }
                    }
                }
            }
        }
    )
}

#[component]
pub fn GuideSidebarCategory(category: GuideCategory<'static>) -> Element {
    rsx! {
        div { class: "flex flex-col gap-1 border-b border-base-200",
            a { href: "{category.page.href}", class: "hover:text-primary",
                h3 { class: "font-bold", "{category.page.meta.title}" }
            }
            ul { class: "flex flex-col gap-1 pb-2",
                for (key, guide) in category.entries.iter().enumerate() {
                    li { key: "{key}", class: "pl-0.5",
                        a { href: guide.href, class: "hover:text-primary",
                            p {
                                "{guide.meta.title}"
                            }
                        }
                    }
                }
            }
        }
    }
}
