use dioxus::prelude::*;

use super::routes::guides::{
    autumn::{highsec::AutumnHighsecGuide, nullsec::AutumnNullsecGuide, AutumnGuide},
    GuidesDirectory,
};
use super::routes::join::JoinAutumn;
use super::routes::tools::AutumnTools;
use super::routes::Layout;
use super::routes::{Home, NotFound};

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

                #[route("/")]
                AutumnGuide {},

                #[nest("/nullsec")]

                    #[route("/")]
                    AutumnNullsecGuide {},

                #[end_nest]

                #[nest("/highsec")]

                    #[route("/")]
                    AutumnHighsecGuide {},

                #[end_nest]

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
