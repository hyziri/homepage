use dioxus::prelude::*;

use crate::web::components::guides::category::GuideCategoryButton;

#[component]
pub fn NullsecGuideCategoryButton() -> Element {
    rsx!(GuideCategoryButton {
        title: "Autumn Nullsec",
        description: "The Order of Autumn",
        image: "https://images.evetech.net/corporations/98785281/logo?size=64".to_string(),
        class: "bg-gradient-to-br from-orange-800 to-amber-800 w-full p-2 text-white rounded",
        image_div_class: "bg-amber-900 w-16"
    })
}

#[component]
pub fn HighsecGuideCategoryButton() -> Element {
    rsx!(GuideCategoryButton {
        title: "Autumn Highsec",
        description: "Autumn Inc.",
        image: "https://images.evetech.net/corporations/98812612/logo?size=64".to_string(),
        class: "bg-gradient-to-br from-sky-800 to-cyan-800 w-full p-2 text-white rounded",
        image_div_class: "bg-cyan-900 w-16"
    })
}
