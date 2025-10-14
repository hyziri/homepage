use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::FaDiscord;
use dioxus_free_icons::icons::fa_solid_icons::FaBars;
use dioxus_free_icons::Icon;
use manganis::Asset;

use crate::web::{
    components::button::discord::AutumnDiscordButton, constant::app::DISCORD_URL, Route,
};

#[component]
pub fn JoinHeader() -> Element {
    const AUTUMN_LOGO: Asset = manganis::asset!(
        "/assets/autumn-logo-dark.avif",
        ImageAssetOptions::new()
            .with_avif()
            .with_size(ImageSize::Manual {
                width: 48,
                height: 48
            })
    );

    rsx! {
        header { class: "fixed w-full flex justify-center bg-base-100 z-20 border-b border-base-200",
            div { class: "navbar max-w-[1440px]",
                ul { class: "navbar-start",
                    Link {
                        class: "btn btn-ghost flex gap-2 items-center font-bold text-2xl",
                        to: Route::Home {  },
                        img {
                            class: "w-12 h-12",
                            alt: "Autumn Logo",
                            src: AUTUMN_LOGO
                        }
                        "Autumn"
                    }
                }
                div { class: "navbar-end",
                    ul { class: "hidden md:flex gap-2",
                        li {
                            AutumnDiscordButton { class: "btn-outline" }
                        }
                        li {
                            Link {
                                to: Route::AutumnJoinGuide {},
                                class: "btn btn-primary",
                                "Begin Your Journey"
                            }
                        }
                    }
                    div { class: "dropdown dropdown-end md:hidden",
                        div {
                            tabindex: 0,
                            role: "button",
                            class: "btn btn-square btn-ghost",
                            Icon { width: 24, height: 24, icon: FaBars }
                        }
                        ul {
                            tabindex: 0,
                            class: "menu dropdown-content bg-base-100 w-52 rounded-b",
                            li {
                                a { href: DISCORD_URL,
                                    Icon { width: 24, height: 24, icon: FaDiscord }
                                    "Autumn Discord"
                                }
                            }
                            li {
                                Link {
                                    to: Route::AutumnJoinGuide {},
                                    "Begin Your Journey"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
