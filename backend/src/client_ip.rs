//! Resolution of the client address behind reverse proxies.

use std::net::IpAddr;

use axum::http::HeaderMap;
use ipnet::IpNet;

/// Returns the address of the client: `X-Forwarded-For` is only honoured when the request
/// comes from a trusted proxy, and only the entries appended by trusted proxies are used,
/// so a client can't spoof its address.
pub fn resolve(peer: IpAddr, headers: &HeaderMap, trusted_proxies: &[IpNet]) -> IpAddr {
    let is_trusted = |ip: &IpAddr| trusted_proxies.iter().any(|net| net.contains(ip));
    if !is_trusted(&peer) {
        return peer;
    }

    let forwarded: Vec<IpAddr> = headers
        .get_all("x-forwarded-for")
        .iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|value| value.split(','))
        .map(|ip| ip.trim().parse::<IpAddr>())
        .collect::<Result<_, _>>()
        // A malformed header can't be trusted at all.
        .unwrap_or_default();

    forwarded
        .into_iter()
        .rev()
        .find(|ip| !is_trusted(ip))
        .unwrap_or(peer)
}

pub fn parse_networks(value: &str) -> anyhow::Result<Vec<IpNet>> {
    value
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.parse::<IpNet>()
                .or_else(|_| s.parse::<IpAddr>().map(IpNet::from))
                .map_err(|_| anyhow::anyhow!("Invalid network: {s}"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use axum::http::HeaderValue;

    use super::*;

    fn headers(values: &[&str]) -> HeaderMap {
        let mut headers = HeaderMap::new();
        for value in values {
            headers.append("x-forwarded-for", HeaderValue::from_str(value).unwrap());
        }
        headers
    }

    fn ip(s: &str) -> IpAddr {
        s.parse().unwrap()
    }

    #[test]
    fn uses_forwarded_address_from_trusted_proxy() {
        let trusted = parse_networks("127.0.0.1/32, ::1").unwrap();
        assert_eq!(
            resolve(ip("127.0.0.1"), &headers(&["203.0.113.7"]), &trusted),
            ip("203.0.113.7")
        );
        assert_eq!(
            resolve(ip("::1"), &headers(&["2001:db8::1"]), &trusted),
            ip("2001:db8::1")
        );
    }

    #[test]
    fn ignores_forwarded_address_from_untrusted_peer() {
        let trusted = parse_networks("127.0.0.1").unwrap();
        assert_eq!(
            resolve(ip("198.51.100.1"), &headers(&["203.0.113.7"]), &trusted),
            ip("198.51.100.1")
        );
    }

    #[test]
    fn spoofed_entries_are_skipped() {
        // The client sent "1.1.1.1" itself, the proxy appended the real address.
        let trusted = parse_networks("127.0.0.1, 10.10.0.0/24").unwrap();
        assert_eq!(
            resolve(
                ip("127.0.0.1"),
                &headers(&["1.1.1.1, 203.0.113.7", "10.10.0.2"]),
                &trusted
            ),
            ip("203.0.113.7")
        );
    }

    #[test]
    fn falls_back_to_peer() {
        let trusted = parse_networks("127.0.0.1").unwrap();
        assert_eq!(
            resolve(ip("127.0.0.1"), &headers(&[]), &trusted),
            ip("127.0.0.1")
        );
        assert_eq!(
            resolve(
                ip("127.0.0.1"),
                &headers(&["garbage, 203.0.113.7"]),
                &trusted
            ),
            ip("127.0.0.1")
        );
    }

    #[test]
    fn rejects_invalid_networks() {
        assert!(parse_networks("nope").is_err());
        assert!(parse_networks("").unwrap().is_empty());
    }
}
