use dioxus::prelude::*;

use crate::web::routes::guides::page::GUIDE_ENTRIES;

#[component]
pub fn GuideSidebar(class: Option<&'static str>) -> Element {
    let class: &str = if let Some(class) = class { class } else { "" };

    rsx!(
        div { class: "{class}",
            div { class: "sticky top-20 z-10",
                h2 { class: "font-bold text-lg", "Autumn Guides" }
                ul { class: "flex flex-col gap-2",
                    for (key, guide) in GUIDE_ENTRIES.iter().enumerate() {
                        li { key: "{key}",
                            a { href: guide.href,
                                p { class: "",
                                    "{guide.meta.title}"
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
