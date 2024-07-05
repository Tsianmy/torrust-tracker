//! Primitive types for [Torrust Tracker](https://docs.rs/torrust-tracker).
//!
//! This module contains the basic data structures for the [Torrust Tracker](https://docs.rs/torrust-tracker),
//! which is a `BitTorrent` tracker server. These structures are used not only
//! by the tracker server crate, but also by other crates in the Torrust
//! ecosystem.
use std::collections::BTreeMap;
use std::time::Duration;

use info_hash::InfoHash;
use serde::{Deserialize, Serialize};

pub mod announce_event;
pub mod info_hash;
pub mod pagination;
pub mod peer;
pub mod swarm_metadata;
pub mod torrent_metrics;

/// Duration since the Unix Epoch.
pub type DurationSinceUnixEpoch = Duration;

/// Serializes a `DurationSinceUnixEpoch` as a Unix timestamp in milliseconds.
/// # Errors
///
/// Will return `serde::Serializer::Error` if unable to serialize the `unix_time_value`.
pub fn ser_unix_time_value<S: serde::Serializer>(unix_time_value: &DurationSinceUnixEpoch, ser: S) -> Result<S::Ok, S::Error> {
    #[allow(clippy::cast_possible_truncation)]
    ser.serialize_u64(unix_time_value.as_millis() as u64)
}

/// IP version used by the peer to connect to the tracker: IPv4 or IPv6
#[derive(PartialEq, Eq, Debug)]
pub enum IPVersion {
    /// <https://en.wikipedia.org/wiki/Internet_Protocol_version_4>
    IPv4,
    /// <https://en.wikipedia.org/wiki/IPv6>
    IPv6,
}

/// Number of bytes downloaded, uploaded or pending to download (left) by the peer.
#[derive(Hash, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumberOfBytes(pub i64);

pub type PersistentTorrents = BTreeMap<InfoHash, u32>;
