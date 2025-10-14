use dioxus::prelude::*;

use crate::web::{components::Section, constant::join::FAQ};

#[component]
pub fn FrequentlyAskedQuestionsSection() -> Element {
    rsx! {
        Section { class: "w-full h-full flex flex-col items-center",
            div { class: "w-full py-6",
                h1 { class: "text-center font-bold text-2xl xl:text-4xl",
                    "Frequently Asked Questions"
                }
            }
            ul { class: "join join-vertical w-full py-6",
                for (key , value) in FAQ.iter().enumerate() {
                    li { key: "{key}", class: "collapse collapse-arrow join-item border-base-300 border",
                        input { r#type: "radio", name: "faq-accordion", aria_label: "faq-accordion" }
                        div { class: "collapse-title text-xl font-medium", "{value.question}" }
                        div { class: "collapse-content",
                            p { "{value.answer}" }
                        }
                    }
                }
            }
        }
    }
}
