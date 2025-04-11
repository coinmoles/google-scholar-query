use regex::Regex;
use reqwest::IntoUrl;
use scraper::{Html, Selector};

use super::{Args, Error, ScholarResult};

#[derive(Debug, Clone, Default)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new(client: reqwest::Client) -> Self {
        Client { client }
    }

    async fn get_document<U: IntoUrl>(&self, url: U) -> Result<String, Error> {
        Ok(self.client.get(url).send().await?.text().await?)
    }

    fn scrape_serialize(&self, document: String) -> Result<Vec<ScholarResult>, Error> {
        let fragment = Html::parse_document(&document[..]);

        let article_selector =
            Selector::parse(".gs_or").unwrap_or_else(|_| unreachable!("Is a valid selector"));
        let title_selector =
            Selector::parse(".gs_rt").unwrap_or_else(|_| unreachable!("Is a valid selector"));
        let abstract_selector =
            Selector::parse(".gs_rs").unwrap_or_else(|_| unreachable!("Is a valid selector"));
        let long_author_selector =
            Selector::parse(".gs_a").unwrap_or_else(|_| unreachable!("Is a valid selector"));
        let link_selector =
            Selector::parse(".gs_rt a").unwrap_or_else(|_| unreachable!("Is a valid selector"));
        let pdf_link_selector = Selector::parse(".gs_or_ggsm a")
            .unwrap_or_else(|_| unreachable!("Is a valid selector"));
        let actions_selector =
            Selector::parse(".gs_flb").unwrap_or_else(|_| unreachable!("Is a valid selector"));

        let nodes = fragment.select(&article_selector).collect::<Vec<_>>();

        let response = nodes
            .chunks_exact(1)
            .filter_map(|rows| {
                let title = rows[0]
                    .select(&title_selector)
                    .next()?
                    .text()
                    .collect::<String>();
                let r#abstract = rows[0]
                    .select(&abstract_selector)
                    .next()?
                    .text()
                    .collect::<String>();
                let link = rows[0]
                    .select(&link_selector)
                    .next()
                    .and_then(|n| n.value().attr("href"))?;
                let pdf_link = rows[0]
                    .select(&pdf_link_selector)
                    .next()
                    .and_then(|n| n.value().attr("href"))
                    .map(|pdf_link| pdf_link.to_string());
                let long_author = rows[0]
                    .select(&long_author_selector)
                    .next()?
                    .text()
                    .collect::<String>();
                let actions = rows[0]
                    .select(&actions_selector)
                    .next()?
                    .text()
                    .collect::<String>();

                // Author, conference and source
                let long_author_regex = Regex::new(
                    r"(?<post_authors>[ \s]- ((?<conference>.*), )?((?<year>\d{4}) - )?(?<domain>.*))$",
                )
                .unwrap_or_else(|_| unreachable!("Is a valid regex"));
                let long_author_matches = long_author_regex.captures(&long_author)?;

                let author = &long_author
                    [0..(long_author.len() - long_author_matches["post_authors"].len())];
                let conference = long_author_matches
                    .name("conference")
                    .map(|conf| conf.as_str().to_string());
                let domain = &long_author_matches["domain"];
                let year = long_author_matches
                    .name("year")
                    .map(|year| year.as_str().to_string());

                // Citations
                let citations_regex = Regex::new(r"(?<citations>\d+)\u{00A0}")
                    .unwrap_or_else(|_| unreachable!("Is a valid regex"));
                let citations = citations_regex
                    .captures(&actions)
                    .and_then(|matches| matches["citations"].parse().ok());

                let result = ScholarResult::new(
                    title, author, r#abstract, conference, link, pdf_link, domain, year, citations,
                );
                Some(result)
            })
            .collect::<Vec<_>>();

        Ok(response)
    }

    pub async fn scrape_scholar(
        &self,
        args: Box<dyn Args + Send>,
    ) -> Result<Vec<ScholarResult>, Error> {
        let url = args.get_url()?;

        let doc = self.get_document(url).await?;

        self.scrape_serialize(doc)
    }
}

impl From<reqwest::Client> for Client {
    fn from(client: reqwest::Client) -> Self {
        Self::new(client)
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Url;

    use crate::scholar::ScholarArgs;

    use super::*;

    #[test]
    fn build_url_query() {
        let sc = ScholarArgs {
            query: String::from("abcd"),
            cite_id: None,
            from_year: None,
            to_year: None,
            sort_by: None,
            cluster_id: None,
            lang: None,
            lang_limit: None,
            limit: None,
            offset: None,
            adult_filtering: None,
            include_similar_results: None,
            include_citations: None,
        };

        let expected = Url::parse("https://scholar.google.com/scholar?q=abcd").unwrap();

        match sc.get_url() {
            Ok(url) => assert!(url.eq(&expected), "value was {}", url),
            Err(_e) => assert_eq!(false, true),
        }
    }

    #[test]
    fn build_url_all() {
        let sc = ScholarArgs {
            query: String::from("abcd"),
            cite_id: Some(String::from("213123123123")),
            from_year: Some(2018),
            to_year: Some(2021),
            sort_by: Some(0),
            cluster_id: Some(String::from("3121312312")),
            lang: Some(String::from("en")),
            lang_limit: Some(String::from("lang_fr|lang_en")),
            limit: Some(10),
            offset: Some(5),
            adult_filtering: Some(true),
            include_similar_results: Some(true),
            include_citations: Some(true),
        };

        let expected = Url::parse("https://scholar.google.com/scholar?q=abcd&cites=213123123123&as_ylo=2018&as_yhi=2021&scisbd=0&cluster=3121312312&hl=en&lr=lang_fr|lang_en&num=10&start=5&safe=active&filter=1&as_vis=1").unwrap();

        match sc.get_url() {
            Ok(url) => assert!(url.eq(&expected), "value was {}", url),
            Err(_e) => assert_eq!(false, true),
        }
    }

    #[tokio::test]
    async fn scrape_with_query() {
        let sc = ScholarArgs {
            query: String::from("machine-learning"),
            cite_id: None,
            from_year: None,
            to_year: None,
            sort_by: None,
            cluster_id: None,
            lang: None,
            lang_limit: None,
            limit: Some(3),
            offset: Some(0),
            adult_filtering: None,
            include_similar_results: None,
            include_citations: None,
        };
        match sc.get_url() {
            Ok(url) => println!("_URLS {}", url),
            Err(_e) => assert_eq!(false, true),
        }

        let client = Client::default();
        match client.scrape_scholar(Box::from(sc)).await {
            Ok(res) => assert_eq!(res.len(), 3),
            Err(_e) => assert_eq!(true, false),
        }
    }
}
