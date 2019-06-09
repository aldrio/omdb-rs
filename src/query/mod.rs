
use reqwest;

use {Movie, Kind, Plot, Error, SearchResults};
use serde::Serialize;
use std::borrow::Borrow;

mod model;
use self::model::{FindResponse, SearchResponse};

/// A function to create and send a request to OMDb.
fn get_request<I, K, V>(params: I) -> Result<reqwest::Response, Error>
    where I: IntoIterator,
          I::Item: Borrow<(K, V)> + Serialize,
          K: AsRef<str> + Serialize,
          V: AsRef<str> + Serialize
{
    const API_ENDPOINT: &'static str = "https://omdbapi.com";
    const API_VERSION: &'static str = "1";

    let params = params.into_iter().collect::<Vec<_>>();

    let response = reqwest::Client::new()
                    .get(API_ENDPOINT)
                    .query(&[("v", API_VERSION)])
                    .query(&[("r", "json")])
                    .query(&params)
                    .send()?;

    let status = response.status();

    if !status.is_success() {
        return Err(Error::Status(status));
    }

    Ok(response)
}

/// Starts a new `FindQuery` with an imdb_id.
///
/// This can be built upon to add other constraints while
/// finding a movie on OMDb.
/// Use this method when you want to select a single movie by *IMDb ID*.
/// # Examples
///
/// Find a movie using it's IMDb id:
///
/// ```
/// let apikey = std::env::var("OMDB_APIKEY").expect("OMDB_APIKEY must be set");
/// let movie = omdb::imdb_id("tt0032138")
///     .apikey(apikey)
/// 	.year(1939)
/// 	.get()
/// 	.unwrap();
///
/// assert!(movie.title == "The Wizard of Oz");
/// ```
pub fn imdb_id<S: Into<String>>(title: S) -> FindQuery {
    FindQuery { imdb_id: Some(title.into()), ..Default::default() }
}

/// Starts a new `FindQuery` with a title.
///
/// This can be built upon to add other constraints while
/// finding a movie on OMDb.
/// Use this method when you want to select a single movie by *title*.
/// # Examples
///
/// Find a series using it's title:
///
/// ```
/// use omdb::Kind;
/// let apikey = std::env::var("OMDB_APIKEY").expect("OMDB_APIKEY must be set");
///
/// let show = omdb::title("Silicon Valley")
///     .apikey(apikey)
/// 	.year(2014)
/// 	.kind(Kind::Series)
/// 	.get()
/// 	.unwrap();
///
/// assert!(show.imdb_id == "tt2575988");
/// ```
pub fn title<S: Into<String>>(title: S) -> FindQuery {
    FindQuery { title: Some(title.into()), ..Default::default() }
}

/// Starts a new `SearchQuery`.
///
/// This can be built upon to add other constraints while
/// searchign for a movie on OMDb.
/// Use this function when you're trying to select multiple movies
/// that fit a set of constraints.
/// # Examples
///
/// Search for movies:
///
/// ```
/// let apikey = std::env::var("OMDB_APIKEY").expect("OMDB_APIKEY must be set");
/// let movies = omdb::search("batman").apikey(apikey).get().unwrap();
///
/// assert!(movies.total_results > 0);
/// ```
pub fn search<S: Into<String>>(search: S) -> SearchQuery {
    SearchQuery { search: search.into(), ..Default::default() }
}

/// Represents a query being bulit for OMDb.
/// Follows the Builder pattern.
#[derive(Debug)]
pub struct FindQuery {
    // One required
    imdb_id: Option<String>,
    title: Option<String>,

    apikey: Option<String>,

    // Optional
    kind: Option<Kind>,
    year: Option<String>,
    plot: Option<Plot>, // TODO: Season and Episode
}

impl Default for FindQuery {
    fn default() -> FindQuery {
        FindQuery {
            imdb_id: None,
            title: None,
            apikey: None,
            kind: None,
            year: None,
            plot: None,
        }
    }
}

impl FindQuery {
    /// Specify the kind of media.
    pub fn kind(&mut self, kind: Kind) -> &mut FindQuery {
        self.kind = Some(kind);
        self
    }

    /// Specify the year.
    pub fn year<S: ToString>(&mut self, year: S) -> &mut FindQuery {
        self.year = Some(year.to_string());
        self
    }

    pub fn apikey<S: ToString>(&mut self, apikey: S) -> &mut FindQuery {
        self.apikey = Some(apikey.to_string());
        self
    }

    /// Specify the plot length.
    pub fn plot(&mut self, plot: Plot) -> &mut FindQuery {
        self.plot = Some(plot);
        self
    }

    /// Perform OMDb Api request and attempt to find the movie
    /// this `FindQuery` is describing.
    pub fn get(&self) -> Result<Movie, Error> {

        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(i) = self.imdb_id.as_ref() {
            params.push(("i", i.clone()));
        } else if let Some(t) = self.title.as_ref() {
            params.push(("t", t.clone()));
        }

        if let Some(k) = self.apikey.as_ref() {
            params.push(("apikey", k.clone()));
        }

        if let Some(kind) = self.kind.as_ref() {
            let k: &str = (*kind).into();
            params.push(("type", String::from(k)));
        }

        if let Some(year) = self.year.as_ref() {
            params.push(("y", year.clone()));
        }

        if let Some(plot) = self.plot.as_ref() {
            let p: &str = (*plot).into();
            params.push(("plot", String::from(p)));
        }

        // Send our request
        let response: FindResponse = get_request(params)?.json()?;

        // Check if the Api's Response string equals true
        if response.response.to_lowercase() != "true" {
            // Return with the Api's Error field or "undefined" if empty
            return Err(Error::Api(response.error.unwrap_or("undefined".to_owned())));
        }

        Ok(response.into())
    }
}

/// Represents a query being bulit for OMDb.
/// Follows the Builder pattern.
#[derive(Debug)]
pub struct SearchQuery {
    search: String,
    apikey: Option<String>,

    // Optional
    kind: Option<Kind>,
    year: Option<String>,
    page: Option<usize>,
}

impl Default for SearchQuery {
    fn default() -> SearchQuery {
        SearchQuery {
            search: String::new(),
            apikey: None,
            kind: None,
            year: None,
            page: None,
        }
    }
}

impl SearchQuery {

    pub fn apikey<S: ToString>(&mut self, apikey: S) -> &mut SearchQuery {
        self.apikey = Some(apikey.to_string());
        self
    }


    /// Specify the kind of media.
    pub fn kind(&mut self, kind: Kind) -> &mut SearchQuery {
        self.kind = Some(kind);

        self
    }

    /// Specify the year.
    pub fn year<S: ToString>(&mut self, year: S) -> &mut SearchQuery {
        self.year = Some(year.to_string());
        self
    }

    /// Specify the page number.
    pub fn page(&mut self, page: usize) -> &mut SearchQuery {
        self.page = Some(page);
        self
    }

    /// Perform OMDb Api request and attempt to find the movie
    /// this `FindQuery` is describing.
    pub fn get(&self) -> Result<SearchResults, Error> {

        let mut params: Vec<(&str, String)> = Vec::new();

        params.push(("s", self.search.clone()));

        if let Some(k) = self.apikey.as_ref() {
            params.push(("apikey", k.clone()));
        }

        if let Some(kind) = self.kind.as_ref() {
            let k: &str = (*kind).into();
            params.push(("type", String::from(k)));
        }

        if let Some(year) = self.year.as_ref() {
            params.push(("y", year.clone()));
        }

        if let Some(page) = self.page.as_ref() {
            params.push(("page", page.to_string()));
        }

        // Send our request
        let response: SearchResponse = get_request(params)?.json()?;

        // Check if the Api's Response string equals true
        if response.response.to_lowercase() != "true" {
            // Return with the Api's Error field or "undefined" if empty
            return Err(Error::Api(response.error.unwrap_or("undefined".to_owned())));
        }

        Ok(response.into())
    }
}
