use dioxus::prelude::*;
use document::{Meta, Title};

use crate::web::components::{Page, Section};

use super::model::GuideMeta;

/// Formats the content of the guide and the author information
#[component]
pub fn Guide(meta: GuideMeta<'static>, children: Element) -> Element {
    rsx! {
        Title { "{meta.title} | Autumn Guides" }
        Meta {
            name: "description",
            content: meta.description
        }
        Page {
            Section {
                class: "flex",
                // Main body
                div { class: "w-4/5",
                    {children}
                }
                // Author sidebar
                div { class: "w-1/5"

                }
            }
        }
    }
}
