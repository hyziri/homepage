pub mod new_members;
pub mod page;

pub use page::AutumnNullsecGuide;

use crate::web::{
    model::guide::GuideCategory,
    routes::guides::autumn::nullsec::new_members::AUTUMN_NULLSEC_NEW_MEMBERS_GUIDE_CATEGORY,
};

pub static AUTUMN_NULLSEC_GUIDE_CATEGORIES: &[GuideCategory] =
    &[AUTUMN_NULLSEC_NEW_MEMBERS_GUIDE_CATEGORY];
