use dioxus::prelude::*;

#[component]
pub fn Container(class: Option<&'static str>, children: Element) -> Element {
    let class: &str = if let Some(class) = class { class } else { "" };

    rsx!(
        section { class: "flex justify-center",
            div { class: "max-w-[1440px] p-6 w-full h-full {class}",
                {children}
            }
        }
    )
}
