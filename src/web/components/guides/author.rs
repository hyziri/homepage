//! Autumn Homepage Guide Authors
//!
//! Provides the [`GuideAuthor`] struct to define the author for a guide
//!
//! Additionally provides statics representing each author that has written
//! a guide on behalf of Autumn.

use crate::web::model::guide::GuideAuthor;

pub static AUTHOR_HYZIRI: GuideAuthor<'static> = GuideAuthor {
    character_id: 2114794365,
    character_name: "Hyziri",
    title: "CEO",
};
