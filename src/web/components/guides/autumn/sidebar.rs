//! # Autumn Guides Sidebar
//!
//! Provides a variant of the guides sidebar with a nullsec/highsec guides listing switcher

use dioxus::prelude::*;

use super::super::sidebar::GuideSidebarCategory;
use crate::web::{
    components::guides::category::GuideCategoryButton, routes::guides::page::GUIDE_CATEGORIES,
};

#[derive(PartialEq)]
enum SelectedAutumnGuideCategory {
    NULLSEC,
    HIGHSEC,
}

#[component]
pub fn AutumnGuideSidebar(class: Option<&'static str>) -> Element {
    let class: &str = if let Some(class) = class { class } else { "" };

    rsx!(
        div { class: "{class}",
            div { class: "sticky top-20 z-10 flex flex-col gap-2",
                a { href: "/guides/autumn", class: "hover:text-primary",
                    h2 { class: "font-bold text-2xl", "Autumn Guides" }
                }
                NullsecHighsecGuideSwitch {}
                a { href: "/guides/autumn/nullsec", class: "hover:text-primary",
                    h2 { class: "font-bold text-xl", "Autumn Nullsec Guides" }
                }
                ul {
                    for (key, category) in GUIDE_CATEGORIES.iter().enumerate() {
                        li { key: "{key}",
                            GuideSidebarCategory { category: *category }
                        }
                    }
               }
            }
        }
    )
}

#[component]
fn NullsecGuideCategoryButton() -> Element {
    rsx!(GuideCategoryButton {
        title: "Autumn Nullsec",
        description: "The Order of Autumn",
        image: "https://images.evetech.net/corporations/98785281/logo?size=64".to_string(),
        class: "bg-gradient-to-br from-orange-800 to-amber-800 w-full p-2 text-white rounded",
        image_div_class: "bg-amber-900 w-16"
    })
}

#[component]
fn HighsecGuideCategoryButton() -> Element {
    rsx!(GuideCategoryButton {
        title: "Autumn Highsec",
        description: "Autumn Inc.",
        image: "https://images.evetech.net/corporations/98812612/logo?size=64".to_string(),
        class: "bg-gradient-to-br from-sky-800 to-cyan-800 w-full p-2 text-white rounded",
        image_div_class: "bg-cyan-900 w-16"
    })
}

#[component]
pub fn NullsecHighsecGuideSwitch() -> Element {
    let mut selected = use_signal(|| SelectedAutumnGuideCategory::NULLSEC);
    let mut dropdown_active = use_signal(|| false);

    rsx! (
        div {
            // Button to toggle dropdown
            button { class: "w-full hover:invert-[0.05]",
                onclick: move |_| {
                    let dropdown_status = *dropdown_active.read();
                    dropdown_active.set(!dropdown_status)
                },
                if *selected.read() == SelectedAutumnGuideCategory::NULLSEC {
                    NullsecGuideCategoryButton {}
                } else {
                    HighsecGuideCategoryButton {}
                }
            }
            // Dropdown to alternate option
            if *dropdown_active.read() {
                div { class: "absolute left-0 w-full z-50",
                    if *selected.read() == SelectedAutumnGuideCategory::NULLSEC {
                        button { class: "w-full hover:invert-[0.05]",
                            onclick: move |_| {
                                selected.set(SelectedAutumnGuideCategory::HIGHSEC);
                                dropdown_active.set(false);
                            },
                            HighsecGuideCategoryButton {}
                        }
                    } else {
                        button { class: "w-full hover:invert-[0.05]",
                            onclick: move |_| {
                                selected.set(SelectedAutumnGuideCategory::NULLSEC);
                                dropdown_active.set(false);
                            },
                            NullsecGuideCategoryButton {}

                        }
                    }
                }
            }
        }
    )
}
