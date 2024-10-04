pub mod advertiser_config;
pub mod config;
pub mod discoverer_config;
pub mod discovery;

pub use advertiser_config::AdvertiserConfig;
use bytes::Bytes;
pub use config::Config;
use core::fmt::Debug;
pub use discoverer_config::DiscovererConfig;
pub use discovery::Advertisement;
use std::net::IpAddr;

pub trait Advertiser {
    fn ip(&self) -> IpAddr;
    fn port(&self) -> u16;
    fn service_name(&self) -> Option<String>;
    fn version(&self) -> Option<String>;
    fn capture_interval(&self) -> u64;
    fn advertisement(&self) -> Bytes;
}

pub trait Discoverer {
    fn ip(&self) -> IpAddr;
    fn port(&self) -> u16;
    fn service_name(&self) -> Option<String>;
    fn version(&self) -> Option<String>;
    fn capture_interval(&self) -> u64;
    fn advert_extract(&self) -> Bytes;
}
