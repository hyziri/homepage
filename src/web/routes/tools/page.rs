use dioxus::prelude::*;
use dioxus_document::{Meta, Title};
use dioxus_free_icons::{icons::fa_brands_icons::FaRust, Icon, IconShape};

use crate::web::components::{Page, Section};

#[derive(PartialEq, Clone)]
struct ToolCardData<T: IconShape + Clone + PartialEq + 'static> {
    icon: T,
    name: &'static str,
    description: &'static str,
    link: &'static str,
}

static DEVELOPER_TOOLS: &[ToolCardData<FaRust>] = &[ToolCardData {
    icon: FaRust,
    name: "eve_esi",
    description: "ESI client for Rust",
    link: "https://github.com/hyziri/eve_esi",
}];

#[component]
pub fn AutumnTools() -> Element {
    rsx! {
        Title {"Tools | Autumn"}
        Meta {
            name: "description",
            content: "Tools for EVE Online, by Autumn."
        }
        Page {
            Section { class: "flex flex-col items-center gap-6 w-full h-full",
                h1 { class: "text-4xl font-bold",
                    "Autumn Tools"
                }
                div { class: "flex w-full",
                    ul { class: "w-full lg:w-1/2",
                        li { class: "p-4",
                            ToolCategory { name: "Developer Tools", tools: DEVELOPER_TOOLS.to_vec() }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ToolCategory<T: IconShape + Clone + PartialEq + 'static>(
    name: String,
    tools: Vec<ToolCardData<T>>,
) -> Element {
    rsx! {
        div {
            h2 { class: "bg-accent text-white p-2 rounded font-bold", "{name}" }
            ul { class: "w-full pt-2",
                for (key, tool) in tools.iter().enumerate() {
                    li { key: "{key}", class: "w-full lg:w-1/2 gap-4",
                        ToolCard { icon: tool.icon.clone(), name: tool.name, description: tool.description, link: tool.link }
                    }
                }

            }
        }
    }
}

#[component]
fn ToolCard<T: IconShape + Clone + PartialEq + 'static>(
    icon: T,
    name: String,
    description: String,
    link: String,
) -> Element {
    rsx! {
        a { href: "{link}", class: "w-full hover:invert-[0.1]",
            div {
                class: "flex gap-2 bg-base-300 rounded p-2",
                div { class: "flex items-center justify-center",
                    div { class: "bg-base-200 rounded-4xl p-2",
                        Icon { width: 24, height: 24, icon: icon }
                    }
                }
                div {
                    h3 { class: "font-bold", "{name}" }
                    p { "{description}"}
                }
            }
        }

    }
}
