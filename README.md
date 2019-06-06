# [OMDb API](https://www.omdbapi.com) for Rust
[![Build Status](https://travis-ci.org/aldrio/omdb-rs.svg?branch=master)](https://travis-ci.org/aldrio/omdb-rs)
[![crates.io](https://img.shields.io/crates/v/omdb.svg?maxAge=2592000?style=plastic)](https://crates.io/crates/omdb)
[![docs.rs](https://docs.rs/omdb/badge.svg)](https://docs.rs/crate/omdb/)

Search movies, tv shows, and games using The Open Movie Database.

## Examples

Find by title:

```rust
let show = omdb::title("The Wizard of Oz")
	.apikey(APIKEY)
	.year(1939)
	.get().unwrap();

assert!(show.imdb_id == "tt0032138");
```

Find by IMDb ID:

```rust
let movie = omdb::imdb_id("tt0111161")
	.apikey(APIKEY)
    .get().unwrap();

assert!(movie.title == "The Shawshank Redemption");
```

Search movies:

```rust
use omdb::Kind;

let movies = omdb::search("batman")
	.apikey(APIKEY)
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
