pub mod error;

use crate::net::ip_range::IpRanges;
use crate::service::discovery::advertisement::DiscovererAdvertCapture;
use crate::service::discovery::discovery_type::discoverer::error::DiscovererError;
use crate::service::discovery::entity::Advertisement;
use crate::service::discovery::service_discovery::ServiceDiscoveryStateTrait;
use bytes::Bytes;
use core::fmt::{Debug, Formatter};
use semver::Version;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

#[derive(Clone)]
pub struct Discoverer<S, A> {
    ip: IpRanges,
    service_name: Option<String>,
    version: Option<String>,
    interval: Option<u64>,
    advert: Option<A>,
    pub(crate) validator:
        Option<Arc<dyn Fn(&Arc<Bytes>) -> Result<(), DiscovererError> + Send + Sync>>,
    pub(crate) processor: Vec<
        Arc<
            dyn Fn(
                    Arc<Bytes>,
                    Option<&S>,
                ) -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>
                + Send
                + Sync
                + 'static,
        >,
    >,
}

impl<S, A> Discoverer<S, A> {
    pub fn new() -> Self {
        Discoverer {
            ip: IpRanges::default(),
            service_name: None,
            version: None,
            interval: None,
            advert: None,
            validator: None,
            processor: Vec::new(),
        }
    }
    pub fn with_ip<T: Into<IpRanges>>(mut self, ip: Option<T>) -> Self {
        self.ip = ip.map(|t| t.into()).unwrap_or(self.ip);
        self
    }
    pub fn set_ip<T: Into<IpRanges>>(&mut self, ip: T) -> &mut Self {
        self.ip = ip.into();
        self
    }

    pub fn with_service_name(mut self, service_name: Option<String>) -> Self {
        self.service_name = service_name;
        self
    }
    pub fn set_service_name(&mut self, service_name: &str) -> &mut Self {
        self.service_name = Some(service_name.to_string());
        self
    }
    pub fn with_version(mut self, version: Option<String>) -> Self {
        self.version = version;
        self
    }
    pub fn set_version(&mut self, version: &str) -> &mut Self {
        self.version = Some(version.to_string());
        self
    }
    pub fn with_interval(mut self, interval: Option<u64>) -> Self {
        self.interval = interval;
        self
    }
    pub fn set_interval(&mut self, interval: u64) -> &mut Self {
        self.interval = Some(interval);
        self
    }

    pub fn with_validator<
        O: Fn(&Arc<Bytes>) -> Result<(), DiscovererError> + Send + Sync + 'static,
    >(
        mut self,
        validator: O,
    ) -> Self {
        self.validator = Some(Arc::new(validator));
        self
    }

    pub fn set_validator<
        O: Fn(&Arc<Bytes>) -> Result<(), DiscovererError> + Send + Sync + 'static,
    >(
        &mut self,
        validator: O,
    ) -> &mut Self {
        self.validator = Some(Arc::new(validator));
        self
    }
}

impl<S, A> Discoverer<S, A>
where
    S: ServiceDiscoveryStateTrait,
{
    pub fn add_processor<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(Arc<Bytes>, Option<&S>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>
            + Send
            + Sync
            + 'static,
    {
        self.processor.push(Arc::new(f));
        self
    }
}

impl<S, A> Default for Discoverer<S, A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S, A> Debug for Discoverer<S, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Discoverer")
            .field("ip", &self.ip)
            .field("service_name", &self.service_name)
            .field("version", &self.version)
            .field("interval", &self.interval)
            .field("advert", &format!("has_data({})", self.advert.is_some()))
            .field("validator", &"Available".to_string())
            .field("processor", &format!("{} processors", self.processor.len()))
            .finish()
    }
}

impl<S, A> DiscovererAdvertCapture<Advertisement> for Discoverer<S, A> {
    fn advert_capture(self, b: Bytes) -> Result<Advertisement, DiscovererError> {
        Advertisement::try_from(b)
            .map_err(|e| DiscovererError::AdvertParsingError(e.to_string()))
            .and_then(|t| // Check service_name
                {
                    if let (Some(service_name), Some(advert_service_id)) = (&self.service_name, &t.service_id) {
                        if service_name != advert_service_id {
                            return Err(DiscovererError::ServiceNameMismatch);
                        }
                    } else if self.service_name.is_some() {
                        return Err(DiscovererError::InvalidDiscoveryData("Service name is missing in the advertisement".to_string()));
                    }

                    // Check version using semver
                    if let (Some(required_version), Some(advert_version)) = (&self.version, &t.version) {
                        let parsed_required_version = Version::parse(required_version)
                            .map_err(|e| DiscovererError::VersionParsingError(e.to_string()))?;

                        let parsed_advert_version = Version::parse(advert_version)
                            .map_err(|e| DiscovererError::VersionParsingError(e.to_string()))?;

                        // Check if advert version satisfies the required version (exact match)
                        if parsed_advert_version != parsed_required_version {
                            return Err(DiscovererError::VersionMismatch);
                        }
                    } else if self.version.is_some() {
                        return Err(DiscovererError::InvalidDiscoveryData("Version is missing in the advertisement".to_string()));
                    }

                    Ok(t)
                })
    }
}
