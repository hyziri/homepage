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

#[cfg(feature = "web")]
// Function to blur active element, used to close an open CSS focus-based dropdown on click
fn blur_active_element() {
    use wasm_bindgen::JsCast;
    use web_sys::HtmlElement;

    if let Some(win) = web_sys::window() {
        if let Some(doc) = win.document() {
            if let Some(active) = doc.active_element() {
                if let Ok(el) = active.dyn_into::<HtmlElement>() {
                    // ignore any error from blur
                    let _ = el.blur();
                }
            }
        }
    }
}

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
    let nav = navigator();

    rsx! (
        div { class: "dropdown",
            // Button to toggle dropdown
            button { class: "w-full hover:invert-[0.05]",
                tabindex: 0,
                role: "button",
                if *subcategory.read() == AutumnGuideSubcategory::NULLSEC {
                    NullsecGuideCategoryButton {}
                } else {
                    HighsecGuideCategoryButton {}
                }
            }
            div { class: "dropdown-content left-0 w-full z-50",
                tabindex: 0,
                if *subcategory.read() == AutumnGuideSubcategory::NULLSEC {
                    button { class: "w-full hover:invert-[0.05]",
                        onclick: move |_| {
                            #[cfg(feature = "web")]
                            blur_active_element();

                            *subcategory.write() = AutumnGuideSubcategory::HIGHSEC;
                            nav.push(Route::AutumnHighsecGuide {});

                        },
                        HighsecGuideCategoryButton {}
                    }
                } else {
                    button { class: "w-full hover:invert-[0.05]",
                        onclick: move |_| {
                            #[cfg(feature = "web")]
                            blur_active_element();

                            *subcategory.write() = AutumnGuideSubcategory::NULLSEC;
                            nav.push(Route::AutumnNullsecGuide {});
                        },
                        NullsecGuideCategoryButton {}
                    }
                }
            }
    }
    )
}
