#[derive(Clone, PartialEq, Copy)]
pub struct GuideAuthor<'a> {
    pub character_name: &'a str,
    pub character_id: i64,
    pub corporation_name: &'a str,
    pub corporation_id: i64,
    pub alliance_name: &'a str,
    pub alliance_id: i64,
}

#[derive(Clone, Copy, PartialEq)]
pub struct GuideMeta<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub date: &'a str,
    pub author: GuideAuthor<'static>,
}
