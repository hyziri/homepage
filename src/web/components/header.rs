use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaBars;
use dioxus_free_icons::Icon;

use crate::web::components::button::discord::AutumnDiscordButton;
use crate::web::constant::app::DISCORD_URL;
use crate::web::model::app::HeaderLink;
use crate::web::Route;

#[component]
pub fn Header() -> Element {
    const AUTUMN_LOGO: Asset = asset!(
        "/assets/autumn-logo-dark.avif",
        ImageAssetOptions::new()
            .with_avif()
            .with_size(ImageSize::Manual {
                width: 48,
                height: 48
            })
    );

    let links: Vec<HeaderLink> = vec![
        HeaderLink {
            text: "Guides",
            route: Route::GuidesDirectory {},
        },
        HeaderLink {
            text: "Tools",
            route: Route::AutumnTools {},
        },
    ];

    rsx! {
        header { class: "fixed w-full flex justify-center bg-base-100 z-20 border-b border-base-200",
            div { class: "max-w-[1440px] w-full flex items-center justify-between px-6 py-3",
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
                ul { class: "hidden md:flex items-center",
                    for (key , value) in links.iter().enumerate() {
                        li { key: "{key}",
                            Link { class: "btn btn-ghost", to: "{value.route}", "{value.text}" }
                        }
                    }
                }
                div {
                    ul { class: "hidden md:flex gap-2",
                        li {
                            AutumnDiscordButton { class: "btn-outline" }
                        }
                        li {
                            Link {
                                to: Route::JoinAutumn {  },
                                class: "btn px-2 md:px-4 btn-primary",
                                "Join Autumn"
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
                            for (key , value) in links.iter().enumerate() {
                                li { key: "{key}",
                                    Link { to: "{value.route}", "{value.text}" }
                                }
                            }
                            li {
                                a { href: DISCORD_URL, "Autumn Discord" }
                            }
                            li {
                                Link { to: Route::JoinAutumn {  }, "Join Autumn" }
                            }
                        }
                    }
                }
            }
        }
    }
}
