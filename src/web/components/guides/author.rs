//! Autumn Homepage Guide Authors
//!
//! Provides the [`GuideAuthor`] struct to define the author for a guide
//!
//! Additionally provides statics representing each author that has written
//! a guide on behalf of Autumn.

use dioxus::prelude::*;

use crate::web::model::guide::GuideAuthor;

#[component]
pub fn GuideAuthorSegment(author: GuideAuthor<'static>) -> Element {
    rsx!(
        div { class: "flex flex-col gap-4 w-full pb-4 border-b border-base-200",
            p {
                class: "font-bold",
                "Posted by"
            }
            a { class: "hover:invert-[0.1]", href: "https://zkillboard.com/character/{author.character_id}/",
                div { class: "flex gap-4 items-center",
                    div { class: "avatar",
                        div { class: "w-16 rounded-full",
                            img { src: "https://images.evetech.net/characters/{author.character_id}/portrait?size=64" }
                        }
                    }
                    div {
                        b { {author.character_name} }
                        p { {author.title} }
                    }
                }
            }
        }
    )
}
