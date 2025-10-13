use dioxus::prelude::*;

use crate::web::{
    components::guides::autumn::layout::AutumnGuideLayout,
    routes::{
        guides::{
            autumn::{
                highsec::{
                    new_members::{AutumnHighsecGettingStartedGuide, AutumnHighsecNewMembersGuide},
                    AutumnHighsecGuide,
                },
                nullsec::{
                    new_members::{AutumnNullsecGettingStartedGuide, AutumnNullsecNewMembersGuide},
                    AutumnNullsecGuide,
                },
                AutumnGuide,
            },
            GuidesDirectory,
        },
        join::JoinAutumn,
        tools::AutumnTools,
        Home, Layout, NotFound,
    },
};

#[rustfmt::skip]
#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[layout(Layout)]

        #[route("/")]
        Home {},

        #[nest("/guides")]

            #[route("/")]
            GuidesDirectory {},

            #[nest("/autumn")]
            #[layout(AutumnGuideLayout)]

                #[route("/")]
                AutumnGuide {},

                #[nest("/nullsec")]

                    #[route("/")]
                    AutumnNullsecGuide {},

                    #[nest("/new-members")]

                        #[route("/")]
                        AutumnNullsecNewMembersGuide {},

                        #[route("/getting-started")]
                        AutumnNullsecGettingStartedGuide {},

                    #[end_nest]

                #[end_nest]

                #[nest("/highsec")]

                    #[route("/")]
                    AutumnHighsecGuide {},

                    #[nest("/new-members")]

                        #[route("/")]
                        AutumnHighsecNewMembersGuide {},

                        #[route("/getting-started")]
                        AutumnHighsecGettingStartedGuide {},

                    #[end_nest]

                #[end_nest]

            #[end_layout]
            #[end_nest]

        #[end_nest]

        #[route("/tools")]
        AutumnTools {},

        #[route("/:..segments")]
        NotFound { segments: Vec<String> },

    #[end_layout]

    #[route("/join")]
    JoinAutumn {},
}
