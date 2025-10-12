pub mod new_members;
pub mod page;

pub use page::AutumnHighsecGuide;

use crate::web::{
    model::guide::GuideCategory,
    routes::guides::autumn::highsec::new_members::AUTUMN_HIGHSEC_NEW_MEMBERS_GUIDE_CATEGORY,
};

pub static AUTUMN_HIGHSEC_GUIDE_CATEGORIES: &[GuideCategory] =
    &[AUTUMN_HIGHSEC_NEW_MEMBERS_GUIDE_CATEGORY];
