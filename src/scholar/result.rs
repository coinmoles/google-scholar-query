#[derive(Debug)]
pub struct ScholarResult {
    pub title: String,
    pub author: String,
    pub abs: String,
    pub conference: Option<String>,
    pub link: String,
    pub pdf_link: Option<String>,
    pub domain: String,
    pub year: Option<String>,
    pub citations: Option<u64>,
}
