pub mod index;
pub mod joining_autumn;

use crate::web::model::guide::{GuideCategory, GuideEntry};

use index::NEW_MEMBER_GUIDES_META;
use joining_autumn::JOINING_AUTUMN_GUIDE_META;

pub static NEW_MEMBER_GUIDE_ENTRIES: [GuideEntry; 1] = [GuideEntry {
    meta: JOINING_AUTUMN_GUIDE_META,
    href: "/guides/new-members/joining-autumn",
    entries: &[],
}];

pub static NEW_MEMBER_GUIDE_CATEGORY: GuideCategory = GuideCategory {
    page: GuideEntry {
        meta: NEW_MEMBER_GUIDES_META,
        href: "/guides/new-members",
        entries: &[],
    },
    entries: &NEW_MEMBER_GUIDE_ENTRIES,
};
