use dioxus::prelude::*;

use crate::web::model::guide::GuideCategory;

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
