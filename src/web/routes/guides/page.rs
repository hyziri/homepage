use dioxus::prelude::*;
use document::{Meta, Title};

use crate::web::{
    components::{Page, Section},
    model::guide::GuideCategory,
    routes::guides::new_members::NEW_MEMBER_GUIDE_CATEGORY,
};

pub static GUIDE_CATEGORIES: [GuideCategory; 1] = [NEW_MEMBER_GUIDE_CATEGORY];

#[component]
pub fn GuidesDirectory() -> Element {
    rsx! {
        Title {"Guides | Autumn"}
        Meta {
            name: "description",
            content: "Guides by The Order of Autumn"
        }
        Page {
            Section { class: "flex flex-col items-center gap-6",
                h1 { class: "text-4xl font-bold",
                    "Autumn Guides"
                }
                ul { class: "flex flex-col items-center gap-4 w-full",
                    li {
                        a { href: "/guides/autumn", class: "hover:invert-[0.1]",
                            GuideCategoryListing {
                                title: "Autumn",
                                description: "Guides for the EVE Online experience with Autumn",
                                image: "https://images.evetech.net/corporations/98785281/logo?size=128",
                                class: "bg-gradient-to-br from-orange-800 to-amber-800",
                                image_div_class: "bg-amber-900"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn GuideCategoryListing(
    title: String,
    description: String,
    image: String,
    class: Option<String>,
    image_div_class: Option<String>,
) -> Element {
    let class: String = if let Some(class) = class {
        class
    } else {
        "".to_string()
    };

    let image_div_class: String = if let Some(class) = image_div_class {
        class
    } else {
        "".to_string()
    };

    rsx! {
        div { class: "w-full md:w-156 md:h-32 rounded flex gap-2 items-center p-6 text-white {class}",
            div { class: "avatar",
                div { class: "w-20 p-2 rounded-full {image_div_class}",
                    img {
                        src: "{image}"
                    }
                }
            }
            div {
                h2 { class: "font-bold text-lg", "{title}" }
                p { "{description}" }
            }
        }
    }
}
