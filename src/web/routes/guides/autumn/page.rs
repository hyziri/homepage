use dioxus::prelude::*;

use crate::web::{
    components::guides::{
        autumn::button::category::{HighsecGuideCategoryButton, NullsecGuideCategoryButton},
        guide::Guide,
    },
    constant::guide::author::AUTHOR_HYZIRI,
    model::guide::{AutumnGuideState, AutumnGuideSubcategory, GuideMeta, GuideOutline},
    Route,
};

static AUTUMN_GUIDE_META: GuideMeta = GuideMeta {
    route: &Route::AutumnGuide {},
    name: "Autumn Guides",
    title: "Autumn Guides",
    description: "Autumn-specific guides for EVE Online",
    date: "2025-10-13",
    author: AUTHOR_HYZIRI,
};

#[component]
pub fn AutumnGuide() -> Element {
    let about_autumn = GuideOutline::new("About Autumn");
    let guide_categories = GuideOutline::new("Guide Categories");

    let guide_outline = vec![about_autumn.clone(), guide_categories.clone()];

    let nav = use_navigator();

    rsx! {
        Guide {
            meta: AUTUMN_GUIDE_META,
            outline: guide_outline,
            h2 { id: "{about_autumn.id}",
                "{about_autumn.title}"
            }
            ul { class: "flex flex-wrap justify-center gap-2 list-none",
                li {
                    a { href: "https://evemaps.dotlan.net/corp/The_Order_of_Autumn", class: "no-underline hover:invert-[0.1]",
                        div  { class: "flex items-center gap-2 shadow w-72 h-24 rounded font-bold p-2",
                            img {
                                src: "https://images.evetech.net/corporations/98785281/logo?size=64",
                            }
                            p {
                                "The Order of Autumn"
                            }
                        }
                    }
                }
                li {
                    a { href: "https://evemaps.dotlan.net/corp/Autumn_Inc.", class: "no-underline hover:invert-[0.1]",
                        div  { class: "flex items-center gap-2 shadow w-72 h-24 rounded font-bold p-2",
                            img {
                                src: "https://images.evetech.net/corporations/98812612/logo?size=64",
                            }
                            p {
                                "Autumn Inc."
                            }
                        }
                    }
                }
            }
            p {
                "Autumn is real life first & new player focused with a heavy emphasis on organization, community, and high quality IT infrastructure."
            }
            ul {
                li {
                    "Our nullsec corporation, "
                    a { href: "https://evemaps.dotlan.net/corp/The_Order_of_Autumn",
                        "The Order of Autumn"
                    }
                    ", is part of "
                    a { href: "https://black-rose.space/",
                        "Black Rose."
                    }
                    " alliance & Phoenix Coalition based out of the region of "
                    a { href: "https://evemaps.dotlan.net/map/Delve/SG-CTQ,T-IPZB,LUA5-L,4O-239",
                    "Delve"
                    }
                    "."
                }
                li {
                    "Our highsec corporation, "
                    a { href: "https://evemaps.dotlan.net/corp/Autumn_Inc.",
                        "Autumn Inc."
                    }
                    ", is part of "
                    a { href: "https://evemaps.dotlan.net/alliance/Iron_Rose.",
                        "Iron Rose."
                    }
                    " based out of the region of "
                    a { href: "https://evemaps.dotlan.net/map/Khanid",
                        "Khanid"
                    }
                    "."
                }
            }
            h2 { id: "{guide_categories.id}",
                "{guide_categories.title}"
            }
            p {
                b { "Our guides are divided into two categories" }
            }
            ul {
                li { "Nullsec guides specific to The Order of Autumn" }
                li { "Highsec guides specific to Autumn Inc." }
            }
            ul { class: "not-prose flex flex-wrap gap-2 justify-center list-none",
                li { class: "w-72",
                    button { class: "hover:invert-[0.05] w-full",
                        onclick: move |_| {
                            nav.push(Route::AutumnNullsecGuide {});
                            consume_context::<AutumnGuideState>().subcategory.set(AutumnGuideSubcategory::NULLSEC);
                        },
                        NullsecGuideCategoryButton {}
                    }
                }
                li { class: "w-72",
                    button { class: "hover:invert-[0.05] w-full",
                        onclick: move |_| {
                            nav.push(Route::AutumnHighsecGuide {});
                            consume_context::<AutumnGuideState>().subcategory.set(AutumnGuideSubcategory::HIGHSEC);
                        },
                        HighsecGuideCategoryButton {}
                    }
                }
            }
            p {
                "The nullsec guides are more in-depth as the experience in The Order of Autumn is more involved due to the nature of nullsec space and the coordination required to maintain sovereignty."
            }
            p {
                "The highsec guides cover the basics of joining the corporation & getting moved to us for easier participation in corporation activities. Beyond that the corporation is fairly hands off for
                you to explore your own interest in EVE and grow at pace you prefer before you decide to finally make the move to nullsec."
            }
        }
    }
}
