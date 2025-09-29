use dioxus::prelude::*;

use super::routes::guides::{autumn::AutumnGuides, Guides};
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
        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
        #[route("/guides")]
        Guides {},
        #[route("/guides/autumn")]
        AutumnGuides {},
        #[route("/tools")]
        AutumnTools {},
    #[end_layout]
    #[route("/join")]
    JoinAutumn {},
}
