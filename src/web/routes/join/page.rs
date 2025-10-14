use dioxus::prelude::*;
use dioxus_document::{Meta, Title};

use crate::web::components::join::JoinHeader;

use crate::web::components::join::{
    section::{
        CallToActionSection, EndgameSection, FrequentlyAskedQuestionsSection, HeroSection,
        LearningCurveSection, WhatMakesAutumnUniqueSection,
    },
    JoinFooter,
};
use crate::web::components::Page;

#[component]
pub fn JoinAutumn() -> Element {
    rsx! {
        Title { "Join Autumn" }
        Meta {
            name: "description",
            content: "EVE is complicated, Autumn makes it straightforward. There are many twists and turns in the beginning of an EVE journey, why waste time learning the hard way when you can learn the right way?"
        }
        JoinHeader {}
        // Set pt-0 so we can set it in Hero instead
        Page { class: "!pt-0",
            HeroSection {}
            EndgameSection {}
            LearningCurveSection {}
            WhatMakesAutumnUniqueSection {}
            CallToActionSection {}
            FrequentlyAskedQuestionsSection {}
        }
        JoinFooter {}
    }
}
