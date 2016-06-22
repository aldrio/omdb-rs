# OMDb API for Rust
[![Build Status](https://travis-ci.org/aldrio/omdb-rs.svg?branch=master)](https://travis-ci.org/aldrio/omdb-rs)

**[Documentation](https://aldrio.github.io/omdb-rs/omdb/)**

## Examples

Find movie by title:
```rust
let show = omdb::title("The Wizard of Oz")
	.year(1939)
	.get().unwrap();

assert!(show.imdb_id == "tt0032138");
```

Find movie by IMDb ID:
```rust
let movie = omdb::imdb_id("tt0111161")
    .get().unwrap();

assert!(movie.title == "The Shawshank Redemption");
```

Search movies:
```rust
use omdb::Kind;

let movies = omdb::search("batman")
	.kind(Kind::Movie) // Optionalally search only for movies
	.get().unwrap();

assert!(movies.total_results > 0);
```