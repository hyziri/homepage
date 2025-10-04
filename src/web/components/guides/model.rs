#[derive(Clone, PartialEq, Copy)]
pub struct GuideAuthor<'a> {
    pub character_name: &'a str,
    pub character_id: i64,
    pub title: &'a str,
}

#[derive(Clone, Copy, PartialEq)]
pub struct GuideMeta<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub date: &'a str,
    pub author: GuideAuthor<'static>,
}

#[derive(Clone, PartialEq, Copy)]
pub struct GuideEntry<'a> {
    pub meta: GuideMeta<'static>,
    pub href: &'a str,
}
