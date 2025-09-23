use super::router::Route;
use dioxus::prelude::*;
use dioxus_document::{Link, Meta, Stylesheet};

#[component]
pub fn App() -> Element {
    const AUTUMN_LOGO: Asset = asset!(
        "/assets/autumn-logo.avif",
        ImageAssetOptions::new()
            .with_avif()
            .with_size(ImageSize::Manual {
                width: 256,
                height: 256
            })
    );

    rsx! {
        Link {
            rel: "icon",
            href: asset!("/assets/favicon.ico")
        }
        Meta {
            name: "og:image",
            content: AUTUMN_LOGO
        }
        Meta {
            name: "twitter:image",
            content: AUTUMN_LOGO
        }
        Stylesheet {
            href: asset!("/assets/tailwind.css")
        }
        Router::<Route> {}
    }
}
