//! `redis-streams-rs` exposes the [Redis Stream](https://redis.io/commands#stream)
//! functionality as a Trait on top of [`redis-rs`](https://github.com/mitsuhiko/redis-rs).
//!
//! The crate is called `redis_streams`.
//!
//! In order to you use this crate, you'll first want to add it as a github
//! dependency (until I have a chance to publish on crates.io).
//!
//! ```ini
//! [dependencies.redis_streams]
//! git = "https://github.com/grippy/redis-streams-rs.git"
//! ```
//!
//! From here, just unlock the streaming commands prior to instantiating client connections.
//!
//! ```no_run
//! extern crate redis_streams;
//! use redis_streams::{client_open,Connection,StreamCommands};
//! let client = client_open("redis://127.0.0.1/0").unwrap();
//! let mut con = client.get_connection().unwrap();
//! ```
//!
//! This crate also exposes all top-level `redis-rs` types.
//! To pick up all `redis-rs` Commands, just use the `Commands` trait.
//!
//! ```no_run
//! extern crate redis_streams;
//! use redis_streams::{Commands};
//! ```
//!
#![deny(non_camel_case_types)]

#[doc(hidden)]
pub extern crate redis;

pub use redis::{Commands, Connection, RedisResult};

pub use commands::StreamCommands;

pub use types::{
    // stream types
    StreamClaimOptions,
    StreamClaimReply,
    StreamId,
    StreamInfoConsumersReply,
    StreamInfoGroupsReply,
    StreamInfoStreamReply,
    StreamKey,
    StreamMaxlen,
    StreamPendingCountReply,
    StreamPendingData,
    StreamPendingReply,
    StreamRangeReply,
    StreamReadOptions,
    StreamReadReply,
};

mod commands;
mod types;

/// Curry `redis::Client::open` calls.
///
pub fn client_open<T: redis::IntoConnectionInfo>(params: T) -> redis::RedisResult<redis::Client> {
    redis::Client::open(params)
}
