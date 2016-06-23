# [OMDb API](https://www.omdbapi.com) for Rust
[![Build Status](https://travis-ci.org/aldrio/omdb-rs.svg?branch=master)](https://travis-ci.org/aldrio/omdb-rs)
[![Crates.io](https://img.shields.io/crates/v/omdb.svg?maxAge=2592000?style=plastic)](https://crates.io/crates/omdb)

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
	.kind(Kind::Movie) // Optionally filter results to movies only
	.get().unwrap();

assert!(movies.total_results > 0);
```

## Usage
Add the crates.io `omdb` dependency to your Cargo.toml file.
```toml

[dependencies]
omdb = "*"

```
Include `extern crate: omdb;` in your crate root.
