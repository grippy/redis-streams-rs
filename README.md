# Note

This project will be archived soon. All of the functionality here was [rolled into redis-rs](https://github.com/mitsuhiko/redis-rs/pull/319). Thanks @Terkwood for making this happen!

# redis-streams-rs

[![Build Status](https://travis-ci.org/grippy/redis-streams-rs.svg?branch=master)](https://travis-ci.org/grippy/redis-streams-rs)

Implements the redis stream trait for `redis-rs` Rust client. This currently requires running code from `redis-rs` master (still waiting on a release to be cut and pushed up to [Crates.io](https://crates.io/crates/redis)).

## Usage

To use `redis-streams-rs`, add this to your `Cargo.toml`:

```toml
[dependencies]
redis-streams = "0.1.0"
```

## See redis-rs for details
[![Build Status](https://travis-ci.org/mitsuhiko/redis-rs.svg?branch=master)](https://travis-ci.org/mitsuhiko/redis-rs)

- [Source](https://github.com/mitsuhiko/redis-rs)
- [Docs](https://mitsuhiko.github.io/redis-rs/redis/)

# Docs

run `make doc` to read the documentation.
