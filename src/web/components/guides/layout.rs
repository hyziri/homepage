use dioxus::prelude::*;
use document::{Meta, Title};

use crate::web::{
    components::{
        guides::{model::GuideAuthor, sidebar::GuideSidebar},
        Page, Section,
    },
    model::breadcrumb::Breadcrumb,
    util::breadcrumb::path_to_breadcrumbs,
};

use super::model::GuideMeta;

/// Formats the content of the guide and the author information
#[component]
pub fn Guide(meta: GuideMeta<'static>, children: Element) -> Element {
    let path = router().full_route_string();

    let breadcrumbs: Vec<Breadcrumb> = path_to_breadcrumbs(path);

    rsx! {
        Title { "{meta.title} | Autumn Guides" }
        Meta {
            name: "description",
            content: meta.description
        }
        Page {
            Section {
                class: "flex min-h-screen",
                GuideSidebar {  class: "w-1/5",

                }
                div { class: "w-3/5",
                    div { class: "flex flex-col",
                        div { class: "breadcrumbs text-sm",
                            ul {
                                for (key, crumb) in breadcrumbs.iter().enumerate() {
                                    li { key: "{key}",
                                        a {
                                            href: "{crumb.href}", "{crumb.name}"
                                        }
                                    }
                                }
                            }
                        }
                        h1 { class: "font-bold text-xl xl:text-2xl", {meta.title} }
                    }
                    article {
                        {children}
                    }
                }
                div { class: "w-1/5",
                    div { class: "sticky top-20 z-10",
                        GuideAuthorSegment { author: meta.author, }
                    }
                }
            }
        }
    }
}

#[component]
pub fn GuideAuthorSegment(author: GuideAuthor<'static>) -> Element {
    rsx!(
        div { class: "flex flex-col gap-4 w-full",
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
                        p { {author.character_name} }
                        p { {author.title} }
                    }
                }
            }
        }
    )
}
