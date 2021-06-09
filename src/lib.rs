//! OMDb API for Rust
//!
//! [Github Repo](https://github.com/aldrio/omdb-rs)
use serde::{Deserialize, Serialize};

mod error;
pub use error::Error;

pub mod query;
pub use query::Episode;
pub use query::imdb_id;
pub use query::search;
pub use query::title;

/// A movie, series, episode, or game from OMDb.
#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub year: String,
    pub rated: String,
    pub released: String,
    pub runtime: String,
    pub genre: String,
    pub director: String,
    pub writer: String,
    pub actors: String,
    pub plot: String,
    pub language: String,
    pub country: String,
    pub awards: String,
    pub poster: String,
    pub metascore: String,
    pub imdb_rating: String,
    pub imdb_votes: String,
    pub imdb_id: String,
    pub kind: Kind,
    pub season: u16,
    pub total_seasons: u16,
    pub episodes: Vec<Episode>,
}

/// Search results from OMDb.
#[derive(Debug)]
pub struct SearchResults {
    pub results: Vec<SearchResultsMovie>,
    pub total_results: usize,
}

/// A movie from an OMDb search.
///
/// These contain less information than a regular `Movie`.
#[derive(Debug)]
pub struct SearchResultsMovie {
    pub title: String,
    pub year: String,
    pub imdb_id: String,
    pub poster: String,
    pub kind: Kind,
}

/// Distinguishes between the different types of media available.
///
/// Note that `Kind` is the same thing as OMDb's `Type`.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Kind {
    Movie,
    Series,
    Episode,
    Game,
}

impl Kind {
    fn from_str(from: &str) -> Option<Kind> {
        match from {
            "movie" => Some(Kind::Movie),
            "series" => Some(Kind::Series),
            "episode" => Some(Kind::Episode),
            "game" => Some(Kind::Game),
            _ => None,
        }
    }
}

impl From<Kind> for &'static str {
    fn from(kind: Kind) -> &'static str {
        match kind {
            Kind::Movie => "movie",
            Kind::Series => "series",
            Kind::Episode => "episode",
            Kind::Game => "game",
        }
    }
}

/// Plot length.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Plot {
    Short,
    Full,
}

impl From<Plot> for &'static str {
    fn from(plot: Plot) -> &'static str {
        match plot {
            Plot::Short => "short",
            Plot::Full => "full",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn imdb_id() {
        let apikey = env::var("OMDB_APIKEY").expect("OMDB_APIKEY must be set");
        let movie = super::imdb_id("tt0032138")
            .apikey(apikey)
            .year(1939)
            .get()
            .await
            .unwrap();

        assert!(movie.title == "The Wizard of Oz");
    }

    #[tokio::test]
    async fn title() {
        let apikey = env::var("OMDB_APIKEY").expect("OMDB_APIKEY must be set");
        let show = super::title("silicon valley")
            .apikey(apikey)
            .year(2014)
            .kind(Kind::Series)
            .get()
            .await
            .unwrap();

        assert!(show.imdb_id == "tt2575988");
    }

    #[tokio::test]
    async fn search() {
        let apikey = env::var("OMDB_APIKEY").expect("OMDB_APIKEY must be set");
        let search = super::search("Batman").apikey(apikey).get().await.unwrap();

        assert!(search.total_results > 0);
    }
}
