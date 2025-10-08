use dioxus::prelude::*;

use crate::web::{
    components::guides::{author::AUTHOR_HYZIRI, layout::Guide},
    model::guide::{GuideMeta, GuideOutline},
};

pub static JOINING_AUTUMN_GUIDE_META: GuideMeta<'static> = GuideMeta {
    title: "Joining Autumn",
    description: "Guide on how to join The Order of Autumn",
    date: "2025-10-02",
    author: AUTHOR_HYZIRI,
};

#[component]
pub fn JoiningAutumnGuide() -> Element {
    let how_to_join_autumn_id = "how-to-join-autumn";
    let section_two_id = "section-two";

    let guide_outline = vec![
        GuideOutline {
            title: "How to join Autumn",
            id: how_to_join_autumn_id,
        },
        GuideOutline {
            title: "Section 2",
            id: section_two_id,
        },
    ];

    rsx! {
        Guide {
            meta: JOINING_AUTUMN_GUIDE_META,
            outline: guide_outline,
            h2 { id: how_to_join_autumn_id,
                "How to join Autumn"
            }
            p {
                " Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer erat dolor, placerat consectetur risus eu, convallis interdum felis. Integer eget vehicula dolor, ac ornare ex. Maecenas ornare consequat tellus. Vivamus ultricies nunc ut ipsum pharetra, id venenatis eros lobortis. Praesent porttitor semper nisi eget mattis. Sed et semper magna. Sed faucibus accumsan nibh id rhoncus. Aenean dapibus tempor lobortis. In sem metus, volutpat in sollicitudin vitae, tincidunt quis urna. Suspendisse et leo turpis. Fusce non erat et ipsum fermentum varius. Nullam purus purus, viverra ut metus eget, imperdiet suscipit erat. Aenean ex sapien, sollicitudin sed congue non, rhoncus eu orci. Nam nulla nisi, fermentum et enim id, finibus aliquam magna. Integer vel egestas massa. "
            }
            h2 { id: section_two_id,
                "Section 2"
            }
            p {
                " Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer erat dolor, placerat consectetur risus eu, convallis interdum felis. Integer eget vehicula dolor, ac ornare ex. Maecenas ornare consequat tellus. Vivamus ultricies nunc ut ipsum pharetra, id venenatis eros lobortis. Praesent porttitor semper nisi eget mattis. Sed et semper magna. Sed faucibus accumsan nibh id rhoncus. Aenean dapibus tempor lobortis. In sem metus, volutpat in sollicitudin vitae, tincidunt quis urna. Suspendisse et leo turpis. Fusce non erat et ipsum fermentum varius. Nullam purus purus, viverra ut metus eget, imperdiet suscipit erat. Aenean ex sapien, sollicitudin sed congue non, rhoncus eu orci. Nam nulla nisi, fermentum et enim id, finibus aliquam magna. Integer vel egestas massa. "
            }
        }
    }
}
