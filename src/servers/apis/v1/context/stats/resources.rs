//! API resources for the [`stats`](crate::servers::apis::v1::context::stats)
//! API context.
use serde::{Deserialize, Serialize};

use crate::core::services::statistics::TrackerMetrics;

/// It contains all the statistics generated by the tracker.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Stats {
    // Torrent metrics
    /// Total number of torrents.
    pub torrents: u64,
    /// Total number of seeders for all torrents.
    pub seeders: u64,
    /// Total number of peers that have ever completed downloading for all torrents.
    pub completed: u64,
    /// Total number of leechers for all torrents.
    pub leechers: u64,

    // Protocol metrics
    /// Total number of TCP (HTTP tracker) connections from IPv4 peers.
    /// Since the HTTP tracker spec does not require a handshake, this metric
    /// increases for every HTTP request.
    pub tcp4_connections_handled: u64,
    /// Total number of TCP (HTTP tracker) `announce` requests from IPv4 peers.
    pub tcp4_announces_handled: u64,
    /// Total number of TCP (HTTP tracker) `scrape` requests from IPv4 peers.
    pub tcp4_scrapes_handled: u64,
    /// Total number of TCP (HTTP tracker) connections from IPv6 peers.
    pub tcp6_connections_handled: u64,
    /// Total number of TCP (HTTP tracker) `announce` requests from IPv6 peers.
    pub tcp6_announces_handled: u64,
    /// Total number of TCP (HTTP tracker) `scrape` requests from IPv6 peers.
    pub tcp6_scrapes_handled: u64,
    /// Total number of UDP (UDP tracker) connections from IPv4 peers.
    pub udp4_connections_handled: u64,
    /// Total number of UDP (UDP tracker) `announce` requests from IPv4 peers.
    pub udp4_announces_handled: u64,
    /// Total number of UDP (UDP tracker) `scrape` requests from IPv4 peers.
    pub udp4_scrapes_handled: u64,
    /// Total number of UDP (UDP tracker) `connection` requests from IPv6 peers.
    pub udp6_connections_handled: u64,
    /// Total number of UDP (UDP tracker) `announce` requests from IPv6 peers.
    pub udp6_announces_handled: u64,
    /// Total number of UDP (UDP tracker) `scrape` requests from IPv6 peers.
    pub udp6_scrapes_handled: u64,
}

impl From<TrackerMetrics> for Stats {
    fn from(metrics: TrackerMetrics) -> Self {
        Self {
            torrents: metrics.torrents_metrics.torrents,
            seeders: metrics.torrents_metrics.complete,
            completed: metrics.torrents_metrics.downloaded,
            leechers: metrics.torrents_metrics.incomplete,
            tcp4_connections_handled: metrics.protocol_metrics.tcp4_connections_handled,
            tcp4_announces_handled: metrics.protocol_metrics.tcp4_announces_handled,
            tcp4_scrapes_handled: metrics.protocol_metrics.tcp4_scrapes_handled,
            tcp6_connections_handled: metrics.protocol_metrics.tcp6_connections_handled,
            tcp6_announces_handled: metrics.protocol_metrics.tcp6_announces_handled,
            tcp6_scrapes_handled: metrics.protocol_metrics.tcp6_scrapes_handled,
            udp4_connections_handled: metrics.protocol_metrics.udp4_connections_handled,
            udp4_announces_handled: metrics.protocol_metrics.udp4_announces_handled,
            udp4_scrapes_handled: metrics.protocol_metrics.udp4_scrapes_handled,
            udp6_connections_handled: metrics.protocol_metrics.udp6_connections_handled,
            udp6_announces_handled: metrics.protocol_metrics.udp6_announces_handled,
            udp6_scrapes_handled: metrics.protocol_metrics.udp6_scrapes_handled,
        }
    }
}

#[cfg(test)]
mod tests {
    use torrust_tracker_primitives::torrent_metrics::TorrentsMetrics;

    use super::Stats;
    use crate::core::services::statistics::TrackerMetrics;
    use crate::core::statistics::Metrics;

    #[test]
    fn stats_resource_should_be_converted_from_tracker_metrics() {
        assert_eq!(
            Stats::from(TrackerMetrics {
                torrents_metrics: TorrentsMetrics {
                    complete: 1,
                    downloaded: 2,
                    incomplete: 3,
                    torrents: 4
                },
                protocol_metrics: Metrics {
                    tcp4_connections_handled: 5,
                    tcp4_announces_handled: 6,
                    tcp4_scrapes_handled: 7,
                    tcp6_connections_handled: 8,
                    tcp6_announces_handled: 9,
                    tcp6_scrapes_handled: 10,
                    udp4_connections_handled: 11,
                    udp4_announces_handled: 12,
                    udp4_scrapes_handled: 13,
                    udp6_connections_handled: 14,
                    udp6_announces_handled: 15,
                    udp6_scrapes_handled: 16
                }
            }),
            Stats {
                torrents: 4,
                seeders: 1,
                completed: 2,
                leechers: 3,
                tcp4_connections_handled: 5,
                tcp4_announces_handled: 6,
                tcp4_scrapes_handled: 7,
                tcp6_connections_handled: 8,
                tcp6_announces_handled: 9,
                tcp6_scrapes_handled: 10,
                udp4_connections_handled: 11,
                udp4_announces_handled: 12,
                udp4_scrapes_handled: 13,
                udp6_connections_handled: 14,
                udp6_announces_handled: 15,
                udp6_scrapes_handled: 16
            }
        );
    }
}
