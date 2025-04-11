use std::borrow::Cow;

use reqwest::Url;

use super::Error;

pub trait Args {
    fn get_service(&self) -> Services;
    fn get_url(&self) -> Result<Url, Error>;
    fn get_limit(&self) -> usize;
}

#[derive(Debug)]
pub enum Services {
    Scholar,
}

impl Services {
    pub fn get_base_url(&self) -> Url {
        match self {
            Services::Scholar => Url::parse("https://scholar.google.com/scholar?")
                .unwrap_or_else(|_| unreachable!("Is a valid URL")),
        }
    }
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
        Services::Scholar
    }

    fn get_url(&self) -> Result<Url, Error> {
        let mut url = self.get_service().get_base_url();

        let query_params = [
            ("q", Some(Cow::Borrowed(self.query.as_str()))),
            ("cites", self.cite_id.as_deref().map(Into::into)),
            ("as_ylo", self.from_year.map(own_display)),
            ("as_yhi", self.to_year.map(own_display)),
            (
                "scisbd",
                self.sort_by
                    .and_then(|s| if s < 3 { Some(own_display(s)) } else { None }),
            ),
            ("cluster", self.cluster_id.as_deref().map(Into::into)),
            ("hl", self.lang.as_deref().map(Into::into)), // TODO: validation
            ("lr", self.lang_limit.as_deref().map(Into::into)),
            ("num", self.limit.map(own_display)),
            ("start", self.offset.map(own_display)),
            ("safe", self.adult_filtering.map(bool_flag("active", "off"))),
            (
                "filter",
                self.include_similar_results.map(bool_flag("1", "0")),
            ),
            ("as_vis", self.include_citations.map(bool_flag("1", "0"))),
        ]
        .into_iter()
        .filter_map(|(k, v)| if let Some(v) = v { Some((k, v)) } else { None });

        url.query_pairs_mut().extend_pairs(query_params);

        return Ok(url);
    }

    fn get_limit(&self) -> usize {
        if let Some(s) = self.limit {
            return s as usize;
        }

        0
    }
}

fn own_display<T: ToString>(v: T) -> Cow<'static, str> {
    Cow::Owned(v.to_string())
}

fn bool_flag<'a>(true_val: &'a str, false_val: &'a str) -> impl Fn(bool) -> Cow<'a, str> {
    move |value| Cow::Borrowed(if value { true_val } else { false_val })
}
