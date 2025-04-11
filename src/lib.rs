pub mod scholar;

#[cfg(test)]
mod tests {
    use crate::scholar;
    #[test]
    fn new_scholar_query() {
        let sc = scholar::ScholarArgs {
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
        assert_eq!(sc.query, "machine-learning");
    }

    #[tokio::test]
    async fn scrape() {
        let sc = scholar::ScholarArgs {
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

        let client = scholar::init_client();
        match client.scrape_scholar(Box::from(sc)).await {
            Ok(result) => assert_eq!(result.len(), 3),
            Err(_e) => assert_eq!(true, false),
        };
    }
}
