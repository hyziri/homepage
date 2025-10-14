use dioxus::prelude::*;
use manganis::Asset;

use crate::web::{
    components::button::discord::AutumnDiscordButton, constant::app::BLACK_ROSE_WEBSITE_URL, Route,
};

#[component]
pub fn HeroSection() -> Element {
    const AUTUMN_LOGO: Asset = manganis::asset!(
        "/assets/autumn-logo.avif",
        ImageAssetOptions::new()
            .with_avif()
            .with_size(ImageSize::Manual {
                width: 256,
                height: 256
            })
    );

    rsx! {
        section { class: "w-full h-full pt-[64px] pb-6 flex items-center justify-center bg-gradient-to-br from-orange-950 to-amber-800 min-h-screen",
            div { class: "max-w-[1440px] px-6 w-full h-full flex flex-col items-center",
                div { class: "md:w-3/4 flex flex-col items-center md:items-start md:self-start gap-4",
                    div { class: "flex flex-col text-center md:text-left items-center md:items-start gap-2",
                        img {
                            class: "w-48 h-48 md:w-64 md:h-64",
                            alt: "Autumn Logo",
                            src: AUTUMN_LOGO
                        }
                        h1 { class: "text-white font-bold sm:text-2xl lg:text-3xl xl:text-4xl",
                            "The Order of Autumn"
                        }
                        h2 { class: "text-white sm:text-lg lg:text-xl xl:text-2xl",
                            "An EVE Online nullsec corporation part of "
                            a {
                                class: "link",
                                href: BLACK_ROSE_WEBSITE_URL,
                                "Black Rose"
                            }
                            " alliance & Phoenix Coalition"
                        }
                        p { class: "text-white text-xs md:text-base",
                            "Autumn is real life first & new player focused with a heavy emphasis on organization, community, and high quality IT infrastructure."
                        }
                    }
                    ul { class: "flex flex-wrap gap-2 justify-center",
                        li {
                            AutumnDiscordButton { class: "px-2 md:px-4"}
                        }
                        li {
                            Link {
                                to: Route::JoinAutumn {  },
                                class: "btn btn-primary px-2 md:px-4",
                                "Join Autumn"
                            }
                        }
                    }
                }
            }
        }
    }
}
