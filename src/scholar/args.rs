use async_trait::async_trait;

use super::Error;

#[async_trait]
pub trait Args {
    fn get_service(&self) -> Services;
    fn get_url(&self) -> Result<String, Error>;
    fn get_limit(&self) -> usize;
}

#[derive(Debug)]
pub enum Services {
    Scholar,
}

#[derive(Debug)]
pub struct ScholarArgs {
    /// q - required
    pub query: String,

    /// cites - citaction id to trigger "cited by"
    pub cite_id: Option<String>,

    /// as_ylo - give results from this year onwards
    pub from_year: Option<u16>,

    /// as_yhi
    pub to_year: Option<u16>,

    /// scisbd - 0 for relevence, 1 to include only abstracts, 2 for everything. Default = date
    pub sort_by: Option<u8>,

    /// cluster - query all versions. Use with q and cites prohibited
    pub cluster_id: Option<String>,

    /// hl - eg: hl=en for english
    pub lang: Option<String>,

    /// lr - one or multiple languages to limit the results to
    /// eg: lr=lang_fr|lang_en
    pub lang_limit: Option<String>,

    /// num - max number of results to return
    pub limit: Option<u32>,

    /// start - result offset. Can be used with limit for pagination
    pub offset: Option<u32>,

    /// safe - level of filtering
    /// safe=active or safe=off
    pub adult_filtering: Option<bool>,

    /// filter - whether to give similar/ommitted results
    /// filter=1 for similar results and 0 for ommitted
    pub include_similar_results: Option<bool>,

    /// as_vis - set to 1 for including citations, otherwise 0
    pub include_citations: Option<bool>,
}

impl Args for ScholarArgs {
    fn get_service(&self) -> Services {
        return Services::Scholar;
    }

    fn get_url(&self) -> Result<String, Error> {
        let mut url = String::from(get_base_url(self.get_service()));

        if self.query == "" {
            return Err(Error::RequiredFieldError);
        }

        url.push_str("q=");
        url.push_str(&self.query);

        if let Some(i) = &self.cite_id {
            url.push_str("&cites=");
            url.push_str(i);
        }
        if let Some(i) = self.from_year {
            url.push_str("&as_ylo=");
            url.push_str(&i.to_string()[..]);
        }
        if let Some(i) = self.to_year {
            url.push_str("&as_yhi=");
            url.push_str(&i.to_string()[..]);
        }
        if let Some(i) = self.sort_by {
            if i < 3 {
                url.push_str("&scisbd=");
                url.push_str(&i.to_string()[..]);
            }
        }
        if let Some(i) = &self.cluster_id {
            url.push_str("&cluster=");
            url.push_str(i);
        }
        if let Some(i) = &self.lang {
            // TODO: validation
            url.push_str("&hl=");
            url.push_str(i);
        }
        if let Some(i) = &self.lang_limit {
            // TODO: validation
            url.push_str("&lr=");
            url.push_str(i);
        }
        if let Some(i) = self.limit {
            url.push_str("&num=");
            url.push_str(&i.to_string()[..]);
        }
        if let Some(i) = self.offset {
            url.push_str("&start=");
            url.push_str(&i.to_string()[..]);
        }
        if let Some(i) = self.adult_filtering {
            url.push_str("&safe=");
            if i {
                url.push_str("active");
            } else {
                url.push_str("off");
            }
        }
        if let Some(i) = self.include_similar_results {
            url.push_str("&filter=");
            if i {
                url.push_str("1");
            } else {
                url.push_str("0");
            }
        }
        if let Some(i) = self.include_citations {
            url.push_str("&as_vis=");
            if i {
                url.push_str("1");
            } else {
                url.push_str("0");
            }
        }

        return Ok(url);
    }

    fn get_limit(&self) -> usize {
        if let Some(s) = self.limit {
            return s as usize;
        }

        return 0;
    }
}

fn get_base_url<'a>(service: Services) -> &'a str {
    match service {
        Services::Scholar => "https://scholar.google.com/scholar?",
    }
}
