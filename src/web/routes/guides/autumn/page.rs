use dioxus::prelude::*;
use dioxus_document::{Meta, Title};

#[component]
pub fn AutumnGuides() -> Element {
    rsx! {
        Title {"Guides | Autumn"}
        Meta {
            name: "description",
            content: "The Order of Autumn Guides"
        }
        section { class: "flex items-center justify-center h-screen pt-[64px] pb-6",
            div { class: "max-w-[1440px] p-6 w-full h-full flex flex-col items-center",
                div { class: "flex flex-col gap-4 py-6",
                    h1 { class: "text-center font-bold text-2xl xl:text-4xl pb-4",
                        "Autumn Guides"
                    }
                }
            }
        }
    }
}
