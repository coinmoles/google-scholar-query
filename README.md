## google-scholar-query

The unofficial Google Scholar API

```rust
fn query_google_scholar() {
    let sc = scholar::ScholarArgs{
        query: String::from("machine-learning"),
        cite_id: None,
        from_year: Some(2018),
        to_year: Some(2021),
        sort_by: Some(0),
        cluster_id: None,
        lang: Some(String::from("en")),
        lang_limit: None,
        limit: Some(3),
        offset: Some(0),
        adult_filtering: None,
        include_similar_results: None,
        include_citations: None,
    };
    
    let client = scholar::init_client();
    match client.scrape_scholar( & sc).await {
        Ok(result) => assert_eq ! (result.len(), 3),
        Err(_e) => assert_eq ! (true, false),
    };
}
```
