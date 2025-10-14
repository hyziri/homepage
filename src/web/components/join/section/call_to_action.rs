use dioxus::prelude::*;
use dioxus_charts::LineChart;
use dioxus_free_icons::{
    icons::{fa_brands_icons::FaDiscord, fa_solid_icons::FaUsers},
    Icon,
};

use crate::{
    model::stats::StatsDto,
    web::{
        components::button::discord::AutumnDiscordButton,
        constant::{app::DISCORD_URL, join::AUTUMN_ORDER_CORP_INFO},
        model::join::CorpCardData,
        Route,
    },
};

#[cfg(feature = "web")]
async fn get_stats() -> Result<Vec<StatsDto>, reqwasm::Error> {
    use reqwasm::http::Request;

    let res = Request::get("/api/stats").send().await?;
    let stats = res.json().await?;

    Ok(stats)
}

#[component]
pub fn CallToAction() -> Element {
    #[derive(PartialEq, Clone, Props)]
    struct CorporationCardProps {
        corporation: &'static CorpCardData,
        stats: StatsDto,
    }

    fn CorporationCard(props: CorporationCardProps) -> Element {
        rsx!(
            div { class: "card card-compact shadow h-96 min-w-64 max-w-72",
                div { class: "card-body flex flex-col justify-between items-center text-center",
                    img {
                        class: "avatar w-24 h-24",
                        src: format!(
                            "https://images.evetech.net/corporations/{}/logo?size=128",
                            props.corporation.corporation_id,
                        ),
                        alt: format!("{} Logo", props.corporation.name)
                    }
                    div { class: "flex flex-col gap-2",
                        p { class: "font-bold", "{props.corporation.name}" }
                        p { "{props.corporation.location}" }
                    }
                    ul { class: "flex justify-evenly w-full",
                        li { class: "flex flex-col items-center gap-2 w-1/2",
                            Icon { width: 24, height: 24, icon: FaUsers }
                            p { "Members" }
                            p { "{props.stats.member_count}" }
                        }
                    }
                    Link {
                        to: Route::AutumnJoinGuide {}, class: "btn btn-primary",
                        { props.corporation.cta_text }
                    }
                }
            }
        )
    }

    let stats = use_signal(Vec::<StatsDto>::new);
    let autumn_order_stats = use_signal(StatsDto::default);

    #[cfg(feature = "web")]
    let mut stats = stats;
    #[cfg(feature = "web")]
    let mut autumn_order_stats = autumn_order_stats;

    #[cfg(feature = "web")]
    {
        let future = use_resource(|| async move { get_stats().await });

        use_effect(move || match &*future.read_unchecked() {
            Some(Ok(stats_data)) => {
                autumn_order_stats.set(
                    stats_data
                        .iter()
                        .find(|x| x.corporation_id == 98785281)
                        .into_iter()
                        .max_by(|a, b| a.date.cmp(&b.date))
                        .cloned()
                        .unwrap_or_default(),
                );

                stats.set(stats_data.to_vec());
            }
            Some(Err(_)) => (),
            None => (),
        });
    }

    rsx! {
        section { class: "flex items-center justify-center",
            div { class: "max-w-[1440px] p-6 w-full h-full flex flex-col items-center",
                div { class: "w-full pt-6",
                    h1 { class: "text-center font-bold text-2xl xl:text-4xl",
                        "Begin Your Journey as Early as Right Now"
                    }
                }
                div { class: "w-full flex flex-col items-center xl:w-1/2",
                    h2 { class: "font-bold text-center text-xl md:text-2xl py-6",
                        "Join Many Others in Taking that First Step"
                    }
                    div { class: "card card-compact h-64 sm:h-96 w-full max-w-[696px] shadow",
                        p { class: "text-center pt-4", "Autumn's Member Count" }
                        LineChart {
                            padding_top: 30,
                            padding_left: 50,
                            padding_right: 100,
                            padding_bottom: 140,
                            show_grid_ticks: true,
                            show_dotted_grid: false,
                            series: vec![stats.iter().map(|stat| stat.member_count as f32).collect::<Vec<f32>>()],
                            labels: stats.iter().map(|stat| stat.date.format("%Y-%m-%d").to_string()).collect::<Vec<String>>()
                        }
                    }
                }
                div { class: "w-full py-6 flex justify-center flex-wrap gap-4 md:gap-0",

                    div { class: "w-full xl:w-1/2",
                        h2 { class: "font-bold text-center text-xl md:text-2xl py-6",
                            "Join The Order of Autumn in Nullsec"
                        }
                        ul { class: "flex flex-wrap justify-center",
                            li { class: "py-2 px-8 md:pr-2 md:py-0",
                                CorporationCard { corporation: &AUTUMN_ORDER_CORP_INFO, stats: autumn_order_stats() }
                            }
                        }
                    }
                }
                div { class: "w-full py-6 flex-wrap md:flex-nowrap flex gap-4 md:gap-0",
                    div { class: "w-full flex flex-col items-center",
                        h2 { class: "font-bold text-center text-xl md:text-2xl pb-6",
                            "Chat with a Recruiter"
                        }
                        div { class: "flex flex-col gap-2",
                            h3 { class: "font-bold text-center text-lg", "Reach us Through Either" }
                            div { class: "flex justify-center",
                                AutumnDiscordButton { class: "btn-outline" }
                            }
                            p {
                                "The "
                                b { "Autumn Public" }
                                " in-game chat channel"
                            }
                            p { "EVE Mail or Discord message any of our leadership below" }
                        }
                    }
                    div {
                        span { class: "font-bold text-center text-2xl hidden md:block px-4",
                            "OR"
                        }
                    }
                    div { class: "w-full",
                        h2 { class: "font-bold text-center text-xl md:text-2xl pb-6",
                            "Begin the Application Process"
                        }
                        ul { class: "flex flex-col gap-2 px-2 sm:px-10",
                            li {
                                "1. Submit your application by following the "
                                Link {
                                    to: Route::AutumnJoinGuide {},
                                    class: "link-primary underline",
                                    { "how to join guide" }
                                }
                                "."
                            }
                            li {
                                "2. Wait for a recruiter to review your application, come chat with us in "
                                a { href: DISCORD_URL, class: "link-primary underline",
                                    "Discord"
                                }
                                " or the in-game channel Autumn Public while you wait!"
                            }
                            li {
                                "3. Accept your invitation, most applications are reviewed in less than 24 hours."
                            }
                            li {
                                "4. Follow our Getting Started Guide to get setup and come take part in our community!"
                            }
                        }
                    }
                }
                ul { class: "w-full py-6 flex justify-center",
                    li {
                        div { class: "card card-compact shadow w-72 md:w-96 md:h-48 lg:h-32",
                            div { class: "card-body flex-row gap-4",
                                img {
                                    class: "avatar w-24 h-24 rounded-full",
                                    src: "https://images.evetech.net/characters/2114794365/portrait?size=128",
                                    alt: "Hyziri's portrait"
                                }
                                div { class: "flex flex-col justify-between",
                                    p { class: "font-bold flex items-center h-1/3", "CEO of Autumn" }
                                    p { class: "font-bold flex items-center h-1/3", "Hyziri" }
                                    div { class: "flex items-center gap-1 h-1/3",
                                        Icon { width: 20, height: 20, icon: FaDiscord }
                                        p { "hyziri" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
