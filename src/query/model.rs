use {Movie, Kind, SearchResults, SearchResultsMovie};

#[derive(Debug, Deserialize)]
pub struct FindResponse {
	#[serde(rename="Response")]
	pub response: String,

	#[serde(rename="Error")]
	pub error: Option<String>,

	#[serde(rename="Title")]
	pub title: Option<String>,
	#[serde(rename="Year")]
	pub year: Option<String>,
	#[serde(rename="Rated")]
	pub rated: Option<String>,
	#[serde(rename="Released")]
	pub released: Option<String>,
	#[serde(rename="Runtime")]
	pub runtime: Option<String>,
	#[serde(rename="Genre")]
	pub genre: Option<String>,
	#[serde(rename="Director")]
	pub director: Option<String>,
	#[serde(rename="Writer")]
	pub writer: Option<String>,
	#[serde(rename="Actors")]
	pub actors: Option<String>,
	#[serde(rename="Plot")]
	pub plot: Option<String>,
	#[serde(rename="Language")]
	pub language: Option<String>,
	#[serde(rename="Country")]
	pub country: Option<String>,
	#[serde(rename="Awards")]
	pub awards: Option<String>,
	#[serde(rename="Poster")]
	pub poster: Option<String>,
	#[serde(rename="Metascore")]
	pub metascore: Option<String>,
	#[serde(rename="imdbRating")]
	pub imdb_rating: Option<String>,
	#[serde(rename="imdbVotes")]
	pub imdb_votes: Option<String>,
	#[serde(rename="imdbID")]
	pub imdb_id: Option<String>,
	#[serde(rename="Type")]
	pub kind: Option<String>,
}

impl From<FindResponse> for Movie {
	fn from(find: FindResponse) -> Movie {
		Movie {
			title: find.title.unwrap_or_default(),
			year: find.year.unwrap_or_default(),
			rated: find.rated.unwrap_or_default(),
			released: find.released.unwrap_or_default(),
			runtime: find.runtime.unwrap_or_default(),
			genre: find.genre.unwrap_or_default(),
			director: find.director.unwrap_or_default(),
			writer: find.writer.unwrap_or_default(),
			actors: find.actors.unwrap_or_default(),
			plot: find.plot.unwrap_or_default(),
			language: find.language.unwrap_or_default(),
			country: find.country.unwrap_or_default(),
			awards: find.awards.unwrap_or_default(),
			poster: find.poster.unwrap_or_default(),
			metascore: find.metascore.unwrap_or_default(),
			imdb_rating: find.imdb_rating.unwrap_or_default(),
			imdb_votes: find.imdb_votes.unwrap_or_default(),
			imdb_id: find.imdb_id.unwrap_or_default(),
			kind: match find.kind {
				Some(kind_string) => {
					match Kind::from_str(&kind_string) {
						Some(kind) => kind,
						None => Kind::Movie,
					}
				}
				None => Kind::Movie,
			},
		}
	}
}


#[derive(Debug, Deserialize)]
pub struct SearchResponse {
	#[serde(rename="Response")]
	pub response: String,

	#[serde(rename="Error")]
	pub error: Option<String>,

	#[serde(rename="Search")]
	pub search: Option<Vec<SearchResponseMovie>>,
	#[serde(rename="totalResults")]
	pub total_results: Option<String>,
}

impl From<SearchResponse> for SearchResults {
	fn from(sr: SearchResponse) -> SearchResults {
		SearchResults {
			results: sr.search.unwrap_or_default().into_iter().map(|srm| srm.into()).collect(),
			total_results: sr.total_results.map(|s| s.parse::<usize>().unwrap_or_default()).unwrap_or_default(),
		}
	}
}

#[derive(Debug, Deserialize)]
pub struct SearchResponseMovie {
	#[serde(rename="Title")]
	pub title: Option<String>,
	#[serde(rename="Year")]
	pub year: Option<String>,
	#[serde(rename="imdbID")]
	pub imdb_id: Option<String>,
	#[serde(rename="Type")]
	pub kind: Option<String>,
	#[serde(rename="Poster")]
	pub poster: Option<String>,
}

impl From<SearchResponseMovie> for SearchResultsMovie {
	fn from(srm: SearchResponseMovie) -> SearchResultsMovie {
		SearchResultsMovie {
			title: srm.title.unwrap_or_default(),
			year: srm.year.unwrap_or_default(),
			poster: srm.poster.unwrap_or_default(),
			imdb_id: srm.imdb_id.unwrap_or_default(),
			kind: match srm.kind {
				Some(kind_string) => {
					match Kind::from_str(&kind_string) {
						Some(kind) => kind,
						None => Kind::Movie,
					}
				}
				None => Kind::Movie,
			},
		}
	}
}
