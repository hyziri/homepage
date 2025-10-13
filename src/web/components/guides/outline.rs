use dioxus::prelude::*;

pub use crate::web::model::guide::GuideOutline;

#[component]
pub fn GuideOutlineSegment(outline: Vec<GuideOutline>) -> Element {
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
                            p { "{entry.title}" }
                        }
                    }
                }
            }
        }
    )
}
