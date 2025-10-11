use dioxus::prelude::*;

use crate::web::{
    components::guides::{author::AUTHOR_HYZIRI, guide::Guide},
    model::guide::{GuideMeta, GuideOutline},
};

pub static JOINING_AUTUMN_GUIDE_META: GuideMeta<'static> = GuideMeta {
    title: "Joining Autumn",
    description: "Guide on how to join The Order of Autumn",
    date: "2025-10-08",
    author: AUTHOR_HYZIRI,
};

#[component]
pub fn JoiningAutumnGuide() -> Element {
    let section_one = GuideOutline::new("Section One");
    let section_two = GuideOutline::new("Section Two");
    let section_three = GuideOutline::new("Section Three");

    let guide_outline = vec![
        section_one.clone(),
        section_two.clone(),
        section_three.clone(),
    ];

    rsx! {
        Guide {
            meta: JOINING_AUTUMN_GUIDE_META,
            outline: guide_outline,
            h2 { id: section_one.id,
                {section_one.title}
            }
            p {
                " Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer erat dolor, placerat consectetur risus eu, convallis interdum felis. Integer eget vehicula dolor, ac ornare ex. Maecenas ornare consequat tellus. Vivamus ultricies nunc ut ipsum pharetra, id venenatis eros lobortis. Praesent porttitor semper nisi eget mattis. Sed et semper magna. Sed faucibus accumsan nibh id rhoncus. Aenean dapibus tempor lobortis. In sem metus, volutpat in sollicitudin vitae, tincidunt quis urna. Suspendisse et leo turpis. Fusce non erat et ipsum fermentum varius. Nullam purus purus, viverra ut metus eget, imperdiet suscipit erat. Aenean ex sapien, sollicitudin sed congue non, rhoncus eu orci. Nam nulla nisi, fermentum et enim id, finibus aliquam magna. Integer vel egestas massa. "
            }
            h2 { id: section_two.id,
                {section_two.title}
            }
            p {
                " Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer erat dolor, placerat consectetur risus eu, convallis interdum felis. Integer eget vehicula dolor, ac ornare ex. Maecenas ornare consequat tellus. Vivamus ultricies nunc ut ipsum pharetra, id venenatis eros lobortis. Praesent porttitor semper nisi eget mattis. Sed et semper magna. Sed faucibus accumsan nibh id rhoncus. Aenean dapibus tempor lobortis. In sem metus, volutpat in sollicitudin vitae, tincidunt quis urna. Suspendisse et leo turpis. Fusce non erat et ipsum fermentum varius. Nullam purus purus, viverra ut metus eget, imperdiet suscipit erat. Aenean ex sapien, sollicitudin sed congue non, rhoncus eu orci. Nam nulla nisi, fermentum et enim id, finibus aliquam magna. Integer vel egestas massa. "
            }
            p {
                " Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer erat dolor, placerat consectetur risus eu, convallis interdum felis. Integer eget vehicula dolor, ac ornare ex. Maecenas ornare consequat tellus. Vivamus ultricies nunc ut ipsum pharetra, id venenatis eros lobortis. Praesent porttitor semper nisi eget mattis. Sed et semper magna. Sed faucibus accumsan nibh id rhoncus. Aenean dapibus tempor lobortis. In sem metus, volutpat in sollicitudin vitae, tincidunt quis urna. Suspendisse et leo turpis. Fusce non erat et ipsum fermentum varius. Nullam purus purus, viverra ut metus eget, imperdiet suscipit erat. Aenean ex sapien, sollicitudin sed congue non, rhoncus eu orci. Nam nulla nisi, fermentum et enim id, finibus aliquam magna. Integer vel egestas massa."
            }
            h3 { id: section_three.id,
                {section_three.title}
            }
            p {
                " Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer erat dolor, placerat consectetur risus eu, convallis interdum felis. Integer eget vehicula dolor, ac ornare ex. Maecenas ornare consequat tellus. Vivamus ultricies nunc ut ipsum pharetra, id venenatis eros lobortis. Praesent porttitor semper nisi eget mattis. Sed et semper magna. Sed faucibus accumsan nibh id rhoncus. Aenean dapibus tempor lobortis. In sem metus, volutpat in sollicitudin vitae, tincidunt quis urna. Suspendisse et leo turpis. Fusce non erat et ipsum fermentum varius. Nullam purus purus, viverra ut metus eget, imperdiet suscipit erat. Aenean ex sapien, sollicitudin sed congue non, rhoncus eu orci. Nam nulla nisi, fermentum et enim id, finibus aliquam magna. Integer vel egestas massa. "
                a { href: "/",
                    " This is a Link"
                }
                b {
                    " Bold text "
                }
                u {
                    "Underline text"
                }
            }

        }
    }
}
