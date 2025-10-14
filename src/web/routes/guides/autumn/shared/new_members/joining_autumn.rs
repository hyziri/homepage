use dioxus::prelude::*;

use crate::web::{
    components::{button::discord::AutumnDiscordButton, guides::guide::Guide},
    constant::guide::author::AUTHOR_HYZIRI,
    model::guide::{GuideMeta, GuideOutline},
    Route,
};

pub static AUTUMN_JOIN_GUIDE_META: GuideMeta = GuideMeta {
    route: &Route::AutumnJoinGuide {},
    name: "Joining Autumn",
    title: "How to Join Autumn",
    description:
        "How to join Autumn in highsec with Autumn Inc. or nullsec with The Order of Autumn",
    date: "2025-10-13",
    author: AUTHOR_HYZIRI,
};

#[component]
pub fn AutumnJoinGuide() -> Element {
    let joining_autumn_in_highsec = GuideOutline::new("Joining Autumn in Highsec");

    let guide_outline = vec![joining_autumn_in_highsec.clone()];

    rsx! {
        Guide {
            meta: AUTUMN_JOIN_GUIDE_META,
            outline: guide_outline,
            h2 { id: "{joining_autumn_in_highsec.id}", "{joining_autumn_in_highsec.title}" }
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
            p {
                a { href: "https://evemaps.dotlan.net/corp/Autumn_Inc.",
                    "Autumn Inc."
                }
                " is the highsec division of Autumn part of the alliance "
                a { href: "https://evemaps.dotlan.net/alliance/Iron_Rose.",
                    "Iron Rose."
                }
                ", the highsec alliance of "
                a { href: "https://black-rose.space/",
                    "Black Rose."
                }
                " based out of the system of "
                a { href: "https://evemaps.dotlan.net/map/Khanid/Moro",
                    "Moro"
                }
                " in the "
                a { href: "https://evemaps.dotlan.net/map/Khanid",
                    "Khanid"
                }
                " region."
            }
            p {
                b { "To apply:"}
            }
            ol {
                li {
                    "Press `Shift + S` in-game to focus on the search bar in the top-left corner"
                }
                li {
                    "Search up the name `Autumn Inc.` and press enter"
                }
                li {
                    "Right click `Autumn Inc.` and click `Show info`"
                }
                li {
                    "Click the `Apply to Join` button in the bottom center of the corporation information window and submit an application"
                }
                li {
                    "Wait for your application to be accepted, response time is typically less than 24 hours"
                }
            }
            p {
                b {
                    "While you wait, consider joining the Autumn Discord:"
                }
            }
            AutumnDiscordButton { class: "btn-outline" }
        }
    }
}
