use dioxus::prelude::*;
use dioxus_document::{Meta, Title};

#[component]
pub fn Guides() -> Element {
    rsx! {
        Title {"Guides | Autumn"}
        Meta {
            name: "description",
            content: "The Order of Autumn Guides"
        }
        section { class: "flex items-center justify-center h-screen pt-[64px] pb-6",
            div { class: "max-w-[1440px] p-6 w-full h-full flex flex-col justify-center items-center",
                div { class: "flex flex-col gap-4 py-6",
                    h1 { class: "text-center font-bold text-2xl xl:text-4xl pb-4",
                        "Guides"
                    }
                    ul { class: "flex flex-col gap-4",
                        li {
                            GuideSectionCard { title: "Autumn", href: "/guides/autumn" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn GuideSectionCard(title: String, href: String) -> Element {
    rsx! {
        a { href: "{href}",
            div { class: "card shadow w-72 h-20 md:w-96 flex items-center justify-center",
                h2 { class: "font-bold text-lg", "{title}" }
            }
        }

    }
}
