use regex::Regex;
use scraper::{Html, Selector};

use super::{Args, Error, ScholarResult};

#[derive(Debug, Clone, Default)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new(client: reqwest::Client) -> Client {
        Client { client }
    }

    async fn get_document(&self, url: &str) -> Result<String, Error> {
        let resp = self.client.get(url).send().await;
        if !resp.is_ok() {
            return Err(Error::ConnectionError);
        }
        let val: String = resp.unwrap().text().await.unwrap();
        return Ok(val);
    }

    fn scrape_serialize(&self, document: String) -> Result<Vec<ScholarResult>, Error> {
        let fragment = Html::parse_document(&document[..]);

        let article_selector = Selector::parse(".gs_or").unwrap();
        let title_selector = Selector::parse(".gs_rt").unwrap();
        let abstract_selector = Selector::parse(".gs_rs").unwrap();
        let long_author_selector = Selector::parse(".gs_a").unwrap();
        let link_selector = Selector::parse(".gs_rt a").unwrap();
        let pdf_link_selector = Selector::parse(".gs_or_ggsm a").unwrap();
        let actions_selector = Selector::parse(".gs_flb").unwrap();

        let nodes = fragment.select(&article_selector).collect::<Vec<_>>();

        let response = nodes
            .chunks_exact(1)
            .map(|rows| {
                let title = rows[0].select(&title_selector)
                    .next()
                    .unwrap();
                let link = rows[0].select(&link_selector)
                    .next()
                    .and_then(|n| n.value().attr("href"))
                    .unwrap();
                let pdf_link = rows[0].select(&pdf_link_selector)
                    .next()
                    .and_then(|n| n.value().attr("href"));
                let abs = rows[0].select(&abstract_selector)
                    .next()
                    .unwrap();
                let long_author = rows[0].select(&long_author_selector)
                    .next()
                    .unwrap();
                let actions = rows[0].select(&actions_selector)
                    .next()
                    .unwrap();

                let ti = title.text().collect::<String>();
                let ab = abs.text().collect::<String>();
                let long_au = long_author.text().collect::<String>();
                let li = link.to_string();
                let pdf_li = match pdf_link {
                    None => None,
                    Some(pdf_link) => Some(pdf_link.to_string())
                };
                let ac = actions.text().collect::<String>();

                // Author, conference and source

                let long_author_regex = Regex::new(r"(?<post_authors>[ \s]- ((?<conference>.*), )?((?<year>\d{4}) - )?(?<domain>.*))$").unwrap();
                let long_author_matches = long_author_regex.captures(&long_au).unwrap();

                let au = long_au[0..(long_au.len() - long_author_matches["post_authors"].len())].to_string();
                let conf = match long_author_matches.name("conference") {
                    None => None,
                    Some(conference) => Some(conference.as_str().to_string())
                };
                let yr = match long_author_matches.name("year") {
                    None => None,
                    Some(year) => Some(year.as_str().to_string())
                };
                let dm = long_author_matches["domain"].to_string();

                // Citations

                let citations_regex = Regex::new(r"(?<citations>\d+)\u{00A0}").unwrap();
                let citations = match citations_regex.captures(&ac) {
                    None => None,
                    Some(matches) => Some(matches["citations"].parse().unwrap()),
                };

                ScholarResult {
                    title: ti,
                    author: au,
                    abs: ab,
                    conference: conf,
                    link: li,
                    pdf_link: pdf_li,
                    domain: dm,
                    year: yr,
                    citations,
                }
            }).collect::<Vec<ScholarResult>>();

        Ok(response)
    }

    pub async fn scrape_scholar(
        &self,
        args: Box<dyn Args + Send>,
    ) -> Result<Vec<ScholarResult>, Error> {
        let url: String;
        match args.get_url() {
            Ok(u) => url = u,
            Err(e) => return Err(e),
        };

        let doc: String;
        match self.get_document(&url[..]).await {
            Ok(page) => doc = page,
            Err(e) => return Err(e),
        };

        return match self.scrape_serialize(doc) {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        };
    }
}

impl From<reqwest::Client> for Client {
    fn from(client: reqwest::Client) -> Self {
        Self::new(client)
    }
}

#[cfg(test)]
mod tests {
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

        match sc.get_url() {
            Ok(url) => assert!(
                url.eq("https://scholar.google.com/scholar?q=abcd"),
                "value was {}",
                url
            ),
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
        match sc.get_url() {
            Ok(url) => assert!(
                url.eq("https://scholar.google.com/scholar?q=abcd&cites=213123123123&as_ylo=2018&as_yhi=2021&scisbd=0&cluster=3121312312&hl=en&lr=lang_fr|lang_en&num=10&start=5&safe=active&filter=1&as_vis=1"), "value was {}", url),
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
