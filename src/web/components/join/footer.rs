use dioxus::prelude::*;
use manganis::Asset;

use crate::web::{
    components::button::discord::AutumnDiscordButton, constant::join::EVE_LEGAL_STATEMENT, Route,
};

#[component]
pub fn JoinFooter() -> Element {
    const AUTUMN_LOGO: Asset = manganis::asset!(
        "/assets/autumn-logo-dark.avif",
        ImageAssetOptions::new()
            .with_avif()
            // This is supposed to be 128x128 but for some reason Dioxus returns
            // "the image IMAGE_PATH cannot be displayed because it contains errors"
            // It works as 48x48 for now
            .with_size(ImageSize::Manual {
                width: 48,
                height: 48
            })
    );

    rsx! {
        footer { class: "footer footer-center flex flex-col bg-base-200 text-base-content p-6 md:p-10 justify-center",
            div { class: "max-w-[1440px] w-full",
                aside { class: "flex flex-col items-center",
                    Link { to: Route::Home {  }, class: "flex flex-col items-center",
                        img {
                            class: "w-32 h-32",
                            alt: "Autumn Logo",
                            src: AUTUMN_LOGO
                        }
                        p { class: "text-3xl font-bold", "Autumn" }
                    }
                }
                nav { class: "w-full",
                    ul { class: "flex w-full",
                        li { class: "w-1/2 pr-1 flex justify-end",
                            AutumnDiscordButton {
                                class: "btn-outline px-2 md:px-4"
                            }
                        }
                        li { class: "w-1/2 pl-1 flex justify-start",
                            Link {
                                to: Route::AutumnJoinGuide {},
                                class: "btn btn-primary px-2 md:px-4",
                                "Begin Your Journey"
                            }
                        }
                    }
                }
            }
            p { class: "text-xs", {EVE_LEGAL_STATEMENT}}
        }
    }
}
