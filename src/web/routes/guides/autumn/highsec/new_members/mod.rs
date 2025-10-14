pub mod getting_started;
pub mod page;

pub use getting_started::AutumnHighsecGettingStartedGuide;
pub use page::AutumnHighsecNewMembersGuide;

use crate::web::{
    model::guide::GuideCategory,
    routes::guides::autumn::{
        highsec::new_members::{
            getting_started::AUTUMN_HIGHSEC_GETTING_STARTED_GUIDE_META,
            page::AUTUMN_HIGHSEC_NEW_MEMBERS_GUIDE_META,
        },
        shared::new_members::joining_autumn::AUTUMN_JOIN_GUIDE_META,
    },
};

pub static AUTUMN_HIGHSEC_NEW_MEMBERS_GUIDE_CATEGORY: GuideCategory = GuideCategory {
    page: AUTUMN_HIGHSEC_NEW_MEMBERS_GUIDE_META,
    entries: &[
        AUTUMN_JOIN_GUIDE_META,
        AUTUMN_HIGHSEC_GETTING_STARTED_GUIDE_META,
    ],
};
