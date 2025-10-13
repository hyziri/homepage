use crate::web::Route;

#[derive(PartialEq)]
pub enum AutumnGuideSubcategory {
    NULLSEC,
    HIGHSEC,
}

#[derive(Clone, PartialEq, Copy)]
pub struct GuideAuthor<'a> {
    pub character_name: &'a str,
    pub character_id: i64,
    pub title: &'a str,
}

#[derive(Clone, Copy, PartialEq)]
pub struct GuideMeta<'a> {
    pub route: &'a Route,
    pub title: &'a str,
    pub description: &'a str,
    pub date: &'a str,
    pub author: GuideAuthor<'static>,
}

#[derive(Clone, Copy, PartialEq)]
pub struct GuideCategory<'a> {
    pub page: GuideMeta<'static>,
    pub entries: &'a [GuideMeta<'a>],
}

#[derive(Clone, PartialEq)]
pub struct GuideOutline {
    pub title: String,
    pub id: String,
}

impl GuideOutline {
    /// Creates a new guide outline instance by setting the `id` based upon `title` String
    ///
    /// Title is converted from `Section Title` to `section-title` for the id
    pub fn new(title: &str) -> Self {
        let id = title.replace(" ", "-").to_lowercase();

        Self {
            title: title.to_string(),
            id: id,
        }
    }
}
