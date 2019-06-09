//! OMDb API for Rust
//! 
//! [Github Repo](https://github.com/aldrio/omdb-rs)
extern crate hyper;
extern crate hyper_native_tls;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

mod error;
pub use error::Error;

pub mod query;
pub use query::imdb_id;
pub use query::title;
pub use query::search;

/// A movie, series, episode, or game from OMDb.
#[derive(Debug)]
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
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Kind {
    Movie,
    Series,
    Episode,
    Game,
}

impl Kind {
    fn from_str(from: &str) -> Option<Kind> {
        match from.as_ref() {
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

    use Kind;

    #[test]
    fn imdb_id() {

        let movie = super::imdb_id("tt0032138")
            .year(1939)
            .get()
            .unwrap();

        assert!(movie.title == "The Wizard of Oz");
    }

    #[test]
    fn title() {

        let show = super::title("silicon valley")
            .year(2014)
            .kind(Kind::Series)
            .get()
            .unwrap();

        assert!(show.imdb_id == "tt2575988");
    }

    #[test]
    fn search() {

        let search = super::search("Batman").get().unwrap();

        assert!(search.total_results > 0);
    }
}
