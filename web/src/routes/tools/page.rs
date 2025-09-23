use dioxus::prelude::*;
use dioxus_document::{Meta, Title};
use dioxus_free_icons::{
    icons::fa_solid_icons::{FaBox, FaSatelliteDish},
    Icon, IconShape,
};

#[derive(PartialEq, Clone)]
struct ToolCardData<T: IconShape + Clone + PartialEq + 'static> {
    icon: T,
    name: &'static str,
    description: &'static str,
    link: &'static str,
}

static FEATURED_TOOLS: &[ToolCardData<FaBox>] = &[ToolCardData {
    icon: FaBox,
    name: "eve_esi",
    description: "ESI client for Rust",
    link: "https://github.com/hyziri/eve_esi",
}];

static INTEL_TOOLS: &[ToolCardData<FaSatelliteDish>] = &[ToolCardData {
    icon: FaSatelliteDish,
    name: "D-Scan",
    description: "Parse local list & D-scans",
    link: "/tools/dscan",
}];

#[component]
pub fn AutumnTools() -> Element {
    rsx! {
        Title {"Tools | Autumn"}
        Meta {
            name: "description",
            content: "Tools for EVE Online, by Autumn."
        }
        section { class: "flex items-center justify-center h-screen pt-[64px] pb-6",
            div { class: "max-w-[1440px] px-6 w-full h-full flex flex-col",
                h2 { class: "text-lg font-bold pl-4 pt-4", "Autumn Tools" }
                ul { class: "px-4 pt-4 gap-4",
                    for (key, card) in FEATURED_TOOLS.iter().enumerate() {
                        li { key: "{key}", class: "lg:w-1/4 gap-4",
                            ToolCard { icon: card.icon.clone(), name: card.name, description: card.description, link: card.link }
                        }
                    }
                }
                ul { class: "w-full",
                    li { class: "lg:w-1/2 p-4",
                        ToolCategory { name: "Intel", tools: INTEL_TOOLS.to_vec() }
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
                    li { key: "{key}", class: "lg:w-1/2 gap-4",
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
        a { href: "{link}", class: "w-full",
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
