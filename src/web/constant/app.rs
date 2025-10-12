use dioxus::prelude::*;

use crate::web::model::guide::AutumnGuideSubcategory;

// Global signals
pub static ACTIVE_AUTUMN_GUIDE_SUBCATEGORY: GlobalSignal<AutumnGuideSubcategory> =
    Signal::global(|| AutumnGuideSubcategory::NULLSEC);

// Social Media URLs
pub const DISCORD_URL: &str = "https://discord.gg/HjaGsBBtFg";
pub const GITHUB_URL: &str = "https://github.com/autumn-order";

// Websites
pub const BLACK_ROSE_WEBSITE_URL: &str = "https://black-rose.space";
pub const APPLICATIONS_URL: &str = "https://apply.autumn-order.com";
