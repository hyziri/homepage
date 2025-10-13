use dioxus::prelude::*;

#[component]
pub fn GuideCategoryButton(
    title: String,
    description: String,
    image: String,
    class: Option<String>,
    image_div_class: Option<String>,
) -> Element {
    let class: String = if let Some(class) = class {
        class
    } else {
        "".to_string()
    };

    let image_div_class: String = if let Some(class) = image_div_class {
        class
    } else {
        "".to_string()
    };

    rsx!(
        div { class: "flex items-center gap-2 text-start {class}",
            div { class: "avatar",
                div { class: "p-2 rounded-full {image_div_class}",
                    img {
                        src: "{image}"
                    }
                }
            }
            div {
                h2 { class: "font-bold", "{title}" }
                p { "{description}" }
            }
        }
    )
}
