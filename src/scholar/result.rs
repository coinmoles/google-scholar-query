use derive_new::new;

#[derive(Debug, new)]
pub struct ScholarResult {
    #[new(into)]
    pub title: String,
    #[new(into)]
    pub author: String,
    #[new(into)]
    pub r#abstract: String,
    #[new(into)]
    pub conference: Option<String>,
    #[new(into)]
    pub link: String,
    #[new(into)]
    pub pdf_link: Option<String>,
    #[new(into)]
    pub domain: String,
    #[new(into)]
    pub year: Option<String>,
    pub citations: Option<u64>,
}
