/// Autumn Homepage Guide Authors
///
/// Provides the [`GuideAuthor`] struct to define the author for a guide
///
/// Additionally provides statics representing each author that has written
/// a guide on behalf of Autumn.
use super::model::GuideAuthor;

pub static AUTHOR_HYZIRI: GuideAuthor<'static> = GuideAuthor {
    character_id: 1,
    character_name: "Hyziri",
    corporation_id: 1,
    corporation_name: "The Order of Autumn",
    alliance_id: 1,
    alliance_name: "Black Rose.",
};
