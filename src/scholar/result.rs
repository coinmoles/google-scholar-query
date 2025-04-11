#[derive(Debug)]
pub struct ScholarResult {
    pub title: String,
    pub author: String,
    pub r#abstract: String,
    pub conference: Option<String>,
    pub link: String,
    pub pdf_link: Option<String>,
    pub domain: String,
    pub year: Option<String>,
    pub citations: Option<u64>,
}

impl ScholarResult {
    pub fn new(
        title: impl Into<String>,
        author: impl Into<String>,
        r#abstract: impl Into<String>,
        conference: Option<impl Into<String>>,
        link: impl Into<String>,
        pdf_link: Option<impl Into<String>>,
        domain: impl Into<String>,
        year: Option<impl Into<String>>,
        citations: Option<u64>,
    ) -> Self {
        Self {
            title: title.into(),
            author: author.into(),
            r#abstract: r#abstract.into(),
            conference: conference.map(Into::into),
            link: link.into(),
            pdf_link: pdf_link.map(Into::into),
            domain: domain.into(),
            year: year.map(Into::into),
            citations,
        }
    }
}
