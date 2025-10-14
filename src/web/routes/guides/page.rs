use dioxus::prelude::*;
use document::{Meta, Title};

use crate::web::{
    components::{guides::category::GuideCategoryButton, Page, Section},
    Route,
};

#[component]
pub fn GuidesDirectory() -> Element {
    rsx! {
        Title {"Guides | Autumn"}
        Meta {
            name: "description",
            content: "Guides by The Order of Autumn"
        }
        Page {
            Section { class: "flex flex-col items-center gap-6 w-full h-full",
                h1 { class: "text-4xl font-bold",
                    "Autumn Guides"
                }
                ul { class: "flex flex-col items-center gap-4 w-full",
                    li {
                        Link { to: Route::AutumnGuide {}, class: "hover:invert-[0.1]",
                            GuideCategoryButton {
                                title: "Autumn",
                                description: "Guides for the EVE Online experience with Autumn",
                                image: "https://images.evetech.net/corporations/98785281/logo?size=128",
                                class: "flex w-full md:w-156 md:h-32 p-6 rounded text-white bg-gradient-to-br from-orange-800 to-amber-800",
                                image_div_class: "w-20 bg-amber-900"
                            }
                        }
                    }
                }
            }
        }
    }
}
