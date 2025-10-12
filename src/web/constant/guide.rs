use dioxus::prelude::*;

use crate::web::model::guide::AutumnGuideSubcategory;

pub static ACTIVE_AUTUMN_GUIDE_SUBCATEGORY: GlobalSignal<AutumnGuideSubcategory> =
    Signal::global(|| AutumnGuideSubcategory::NULLSEC);
