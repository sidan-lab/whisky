use std::str::FromStr;

use pallas::{codec::utils::Bytes, ledger::primitives::Relay as PallasRelay};
use whisky_common::WError;

pub enum RelayKind {
    SingleHostAddr(Option<u32>, Option<String>, Option<String>),
    SingleHostName(Option<u32>, String),
    MultiHostName(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Relay {
    pub inner: PallasRelay,
}

impl Relay {
    pub fn new(relay: RelayKind) -> Result<Self, WError> {
        let pallas_relay = match relay {
            RelayKind::SingleHostAddr(port, ipv4, ipv6) => {
                let ipv4_bytes: Option<Bytes> = match &ipv4 {
                    Some(ip) => Some(Bytes::from_str(ip).map_err(|_| {
                        WError::new(
                            "WhiskyPallas - encoding ipv4",
                            "failed to parse ipv4 address",
                        )
                    })?),
                    None => None,
                };
                let ipv6_bytes: Option<Bytes> = match &ipv6 {
                    Some(ip) => Some(Bytes::from_str(ip).map_err(|_| {
                        WError::new(
                            "WhiskyPallas - encoding ipv6",
                            "failed to parse ipv6 address",
                        )
                    })?),
                    None => None,
                };
                PallasRelay::SingleHostAddr(port, ipv4_bytes, ipv6_bytes)
            }
            RelayKind::SingleHostName(port, dns_name) => {
                PallasRelay::SingleHostName(port, dns_name)
            }
            RelayKind::MultiHostName(dns_name) => PallasRelay::MultiHostName(dns_name),
        };
        Ok(Relay {
            inner: pallas_relay,
        })
    }
}
