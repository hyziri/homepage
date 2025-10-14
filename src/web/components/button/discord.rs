use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_brands_icons::FaDiscord, Icon};

use crate::web::constant::app::DISCORD_URL;

#[component]
pub fn AutumnDiscordButton(class: Option<&'static str>) -> Element {
    let class: &str = if let Some(class) = class { class } else { "" };

    rsx!(
        a {  class: "btn text-neutral {class}", href: DISCORD_URL,
            Icon { width: 24, height: 24, icon: FaDiscord }
            "Autumn Discord"
        }
    )
}
