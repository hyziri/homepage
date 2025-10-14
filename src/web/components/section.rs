use dioxus::prelude::*;

#[component]
pub fn Section(class: Option<&'static str>, children: Element) -> Element {
    let class: &str = if let Some(class) = class { class } else { "" };

    rsx!(
        section { class: "max-w-[1440px] p-6 {class}",
            {children}
        }
    )
}
