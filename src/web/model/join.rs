pub struct FAQEntry {
    pub question: &'static str,
    pub answer: &'static str,
}

#[derive(PartialEq)]
pub struct CorpCardData {
    pub name: &'static str,
    pub corporation_id: u64,
    pub location: &'static str,
    pub cta_text: &'static str,
}
