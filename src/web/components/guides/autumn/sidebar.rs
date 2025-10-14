//! # Autumn Guides Sidebar
//!
//! Provides a variant of the guides sidebar with a nullsec/highsec guides listing switcher

use dioxus::prelude::*;

use crate::web::{
    components::guides::{
        autumn::button::category::{HighsecGuideCategoryButton, NullsecGuideCategoryButton},
        sidebar::GuideSidebarCategory,
    },
    model::guide::{AutumnGuideSubcategory, GuideCategory},
    routes::guides::autumn::{
        highsec::AUTUMN_HIGHSEC_GUIDE_CATEGORIES, nullsec::AUTUMN_NULLSEC_GUIDE_CATEGORIES,
    },
    Route,
};

#[component]
pub fn AutumnGuideSidebar(
    class: Option<&'static str>,
    subcategory: Signal<AutumnGuideSubcategory>,
) -> Element {
    let class: &str = if let Some(class) = class { class } else { "" };

    let category_entries: &[GuideCategory] = match *subcategory.read() {
        AutumnGuideSubcategory::NULLSEC => AUTUMN_NULLSEC_GUIDE_CATEGORIES,
        AutumnGuideSubcategory::HIGHSEC => AUTUMN_HIGHSEC_GUIDE_CATEGORIES,
    };

    rsx!(
        div { class: "{class}",
            div { class: "sticky top-20 z-10 flex flex-col gap-2",
                Link { to: Route::AutumnGuide {}, class: "hover:text-primary",
                    h2 { class: "font-bold text-2xl", "Autumn Guides" }
                }
                NullsecHighsecGuideSwitch { subcategory: subcategory }
                if *subcategory.read() == AutumnGuideSubcategory::NULLSEC {
                    Link { to: Route::AutumnNullsecGuide {}, class: "hover:text-primary",
                        h2 { class: "font-bold text-xl", "Autumn Nullsec Guides" }
                    }
                } else {
                    Link { to: Route::AutumnHighsecGuide {}, class: "hover:text-primary",
                        h2 { class: "font-bold text-xl", "Autumn Highsec Guides" }
                    }
                }
                ul {
                    for (key, category) in category_entries.iter().enumerate() {
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
pub fn NullsecHighsecGuideSwitch(subcategory: Signal<AutumnGuideSubcategory>) -> Element {
    let mut dropdown_active = use_signal(|| false);

    let nav = navigator();

    rsx! (
        div {
            // Button to toggle dropdown
            button { class: "w-full hover:invert-[0.05]",
                onclick: move |_| {
                    let dropdown_status = *dropdown_active.read();
                    dropdown_active.set(!dropdown_status)
                },
                if *subcategory.read() == AutumnGuideSubcategory::NULLSEC {
                    NullsecGuideCategoryButton {}
                } else {
                    HighsecGuideCategoryButton {}
                }
            }
            // Dropdown to alternate option
            if *dropdown_active.read() {
                div { class: "absolute left-0 w-full z-50",
                    if *subcategory.read() == AutumnGuideSubcategory::NULLSEC {
                        button { class: "w-full hover:invert-[0.05]",
                            onclick: move |_| {
                                *subcategory.write() = AutumnGuideSubcategory::HIGHSEC;
                                dropdown_active.set(false);
                                nav.push(Route::AutumnHighsecGuide {});
                            },
                            HighsecGuideCategoryButton {}
                        }
                    } else {
                        button { class: "w-full hover:invert-[0.05]",
                            onclick: move |_| {
                                *subcategory.write() = AutumnGuideSubcategory::NULLSEC;
                                dropdown_active.set(false);
                                nav.push(Route::AutumnNullsecGuide {});
                            },
                            NullsecGuideCategoryButton {}
                        }
                    }
                }
            }
        }
    )
}
