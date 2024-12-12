use crate::net::ip::IpAddrs;
use crate::prelude::collections::HashSet;
use crate::rails::ext::blocking::Merge;
use core::fmt::{Display, Formatter};
#[cfg(feature = "regex")]
use regex::Regex;
use std::io;
use std::net::{
    IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr as StdSocketAddr, ToSocketAddrs as StdToSocketAddrs,
};

// Struct representing a single IP address and port binding
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct SocketAddr {
    ip: Option<IpAddr>,
    port: Option<u16>,
    default_ip: Option<IpAddr>,
    default_port: Option<u16>,
}

impl SocketAddr {
    /// Creates a new `SocketBindAddr` with the provided IP and port.
    pub fn new(ip: IpAddr, port: u16) -> Self {
        SocketAddr {
            ip: Some(ip),
            port: Some(port),
            default_ip: None,
            default_port: None,
        }
    }

    /// Detects all available IP addresses on the system.
    /// If there is more than one IP address, it sets the address to 0.0.0.0.
    #[cfg(all(feature = "pnet", target_os = "linux"))]
    pub fn detect_ip(&mut self) {
        let available_ips: Vec<IpAddr> = Self::get_available_ips();

        if available_ips.len() > 1 {
            // If more than one IP is available, set to 0.0.0.0
            self.ip = Some(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
        } else if let Some(ip) = available_ips.first() {
            // Otherwise, set to the single available IP
            self.ip = Some(*ip);
        }
    }

    #[cfg(all(feature = "regex", feature = "pnet", target_os = "linux"))]
    pub fn detect_ip_match(&mut self, pattern: &str, capture_first_valid: bool) {
        let available_ips: Vec<IpAddr> = Self::get_available_ips();
        let re = Regex::new(pattern).expect("Invalid regular expression");

        // If not capturing the first valid IP, check all available IPs against the pattern
        for ip in &available_ips {
            if re.is_match(&ip.to_string()) {
                // Attempt to find the first valid (non-localhost, non-global) IP
                match ip {
                    IpAddr::V4(ip_v4)
                        if *ip_v4 != Ipv4Addr::new(0, 0, 0, 0) && *ip_v4 != Ipv4Addr::LOCALHOST =>
                    {
                        self.ip = Some(*ip);
                        return;
                    }
                    IpAddr::V6(ip_v6)
                        if *ip_v6 != Ipv6Addr::UNSPECIFIED && *ip_v6 != Ipv6Addr::LOCALHOST =>
                    {
                        self.ip = Some(*ip);
                        return;
                    }
                    _ => continue,
                }
            }
        }
    }

    #[cfg(all(feature = "pnet", target_os = "linux"))]
    pub fn with_detect_ip(mut self) -> Self {
        self.detect_ip();
        self
    }

    #[cfg(all(feature = "regex", feature = "pnet", target_os = "linux"))]
    pub fn with_detect_ip_match(mut self, pattern: &str, capture_first_valid: bool) -> Self {
        self.detect_ip_match(pattern, capture_first_valid);
        self
    }

    pub fn new_with_defaults(ip: IpAddr, port: u16, default_ip: IpAddr, default_port: u16) -> Self {
        SocketAddr {
            ip: Some(ip),
            port: Some(port),
            default_ip: Some(default_ip),
            default_port: Some(default_port),
        }
    }
    pub fn as_broadcast_addr(&self, subnet_mask: Option<IpAddr>) -> Result<SocketAddr, String> {
        let mask = subnet_mask
            .and_then(Self::is_valid_subnet_mask)
            .unwrap_or(Self::default_subnet_mask(subnet_mask));

        match (self.ip_with_defaults(), mask) {
            (IpAddr::V4(ip_v4), IpAddr::V4(mask_v4)) => {
                let ip_octets = ip_v4.octets();
                let mask_octets = mask_v4.octets();
                let broadcast_octets = [
                    ip_octets[0] | !mask_octets[0],
                    ip_octets[1] | !mask_octets[1],
                    ip_octets[2] | !mask_octets[2],
                    ip_octets[3] | !mask_octets[3],
                ];
                let broadcast_ip = Ipv4Addr::new(
                    broadcast_octets[0],
                    broadcast_octets[1],
                    broadcast_octets[2],
                    broadcast_octets[3],
                );
                Ok(Self::new(
                    IpAddr::V4(broadcast_ip),
                    self.port_with_defaults(),
                ))
            }
            (IpAddr::V6(ip_v6), IpAddr::V6(mask_v6)) => {
                let ip_segments = ip_v6.segments();
                let mask_segments = mask_v6.segments();
                let mut broadcast_segments = [0u16; 8];

                // Calculate pseudo-broadcast address for IPv6 (using bitwise OR)
                for i in 0..8 {
                    // Invert only the bits corresponding to the host portion of the mask
                    let host_bits = !mask_segments[i];
                    broadcast_segments[i] = ip_segments[i] | host_bits;
                }
                let broadcast_ip = Ipv6Addr::from(broadcast_segments);
                Ok(Self::new(
                    IpAddr::V6(broadcast_ip),
                    self.port_with_defaults(),
                ))
            }
            _ => Err("Mismatched IP and subnet mask types.".to_string()),
        }
    }

    /// Creates a `SocketBindAddr` with default IP and port values.
    /// Default IP: 0.0.0.0, Default Port: 9999
    pub fn default_addr() -> Self {
        Self {
            ip: Some(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
            port: Some(9999),
            default_ip: Some(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
            default_port: Some(9999),
        }
    }

    /// Sets the IP address of the bind address.
    pub fn with_ip(mut self, ip: Option<IpAddr>) -> Self {
        self.ip = ip;
        self
    }

    /// Sets the port of the bind address.
    pub fn with_port(mut self, port: Option<u16>) -> Self {
        self.port = port;
        self
    }

    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.port = Some(port);
        self
    }

    pub fn set_ip(&mut self, ip: IpAddr) -> &mut Self {
        self.ip = Some(ip);
        self
    }

    pub fn port(&self) -> Option<u16> {
        self.port.or_else(|| self.default_port)
    }

    pub fn ip(&self) -> Option<IpAddr> {
        self.ip.or_else(|| self.default_ip)
    }

    pub fn port_with_defaults(&self) -> u16 {
        self.port().unwrap_or(9999)
    }
    pub fn ip_with_defaults(&self) -> IpAddr {
        self.ip().unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)))
    }
    pub fn has_valid_nonglobal_binding(&self) -> bool {
        match self.ip_with_defaults() {
            IpAddr::V4(ip) => ip != Ipv4Addr::new(0, 0, 0, 0) && self.port_with_defaults() != 0,
            IpAddr::V6(ip) => ip != Ipv6Addr::UNSPECIFIED && self.port_with_defaults() != 0,
        }
    }
}

/// Private methods
impl SocketAddr {
    /// Returns the default subnet mask for a given IP address.
    /// If the input is `None`, it returns the default IPv4 mask.
    /// If the input is an IPv4 address, it returns the default IPv4 mask.
    /// If the input is an IPv6 address, it returns the default IPv6 mask.
    pub fn default_subnet_mask(ip: Option<IpAddr>) -> IpAddr {
        match ip {
            Some(IpAddr::V4(_)) => IpAddr::V4(Ipv4Addr::new(255, 0, 0, 0)), // Default IPv4 mask
            Some(IpAddr::V6(_)) => {
                IpAddr::V6(Ipv6Addr::new(0xffff, 0xffff, 0xffff, 0xffff, 0, 0, 0, 0))
            } // Default IPv6 mask
            None => IpAddr::V4(Ipv4Addr::new(255, 0, 0, 0)), // Default to IPv4 mask if None
        }
    }
    /// Helper function to check if a subnet mask is valid.
    fn is_valid_subnet_mask(mask: IpAddr) -> Option<IpAddr> {
        match mask {
            IpAddr::V4(mask_v4) => {
                let mask_num = u32::from(mask_v4);
                // Create the inverse of the mask and check if it's continuous 0s followed by 1s
                let inv_mask = !mask_num;
                let is_continuous = (inv_mask & (inv_mask + 1)) == 0;

                // Check if it's a valid IPv4 subnet mask (continuous 1s followed by 0s)
                if mask_num != 0 && is_continuous && mask_num.leading_zeros() != 32 {
                    Some(IpAddr::V4(mask_v4))
                } else {
                    None
                }
            }
            IpAddr::V6(mask_v6) => {
                let segments = mask_v6.segments();
                let mut found_zero_segment = false;
                let mut mask_valid = true;

                for &segment in segments.iter() {
                    if found_zero_segment {
                        if segment != 0 {
                            mask_valid = false;
                            break;
                        }
                    } else {
                        // If it's not a fully set segment and not zero, check for partial bits
                        if segment != 0xFFFF {
                            // Check for valid bit pattern (continuous 1s followed by 0s)
                            if (segment & (segment + 1)) != 0 {
                                mask_valid = false;
                                break;
                            }
                            found_zero_segment = true;
                        }
                    }
                }

                if mask_valid {
                    Some(IpAddr::V6(mask_v6))
                } else {
                    None
                }
            }
        }
    }

    /// Helper function to get a list of all available IP addresses on the system.
    #[cfg(all(feature = "pnet", target_os = "linux"))]
    fn get_available_ips() -> Vec<IpAddr> {
        let mut ips = Vec::new();
        let interfaces = pnet::datalink::interfaces();
        for interface in interfaces {
            for ip in interface.ips {
                ips.push(ip.ip());
            }
        }

        ips
    }
}

impl Default for SocketAddr {
    fn default() -> Self {
        SocketAddr::default_addr()
    }
}

impl Display for SocketAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SocketBindAddr({}:{})",
            self.ip_with_defaults().to_string(),
            self.port_with_defaults()
        )
    }
}

impl StdToSocketAddrs for SocketAddr {
    type Iter = std::vec::IntoIter<StdSocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        Ok(vec![StdSocketAddr::new(
            self.ip_with_defaults(),
            self.port_with_defaults(),
        )]
        .into_iter())
    }
}

impl From<SocketAddr> for StdSocketAddr {
    fn from(bind_addr: SocketAddr) -> Self {
        StdSocketAddr::new(bind_addr.ip_with_defaults(), bind_addr.port_with_defaults())
    }
}

impl From<&SocketAddr> for StdSocketAddr {
    fn from(bind_addr: &SocketAddr) -> Self {
        StdSocketAddr::new(bind_addr.ip_with_defaults(), bind_addr.port_with_defaults())
    }
}

impl From<SocketAddr> for SocketAddrs {
    fn from(bind_addr: SocketAddr) -> Self {
        let mut set = HashSet::new();
        set.insert(bind_addr);
        SocketAddrs {
            bind_addr: Some(set),
            default_bind_addr: None,
            default_port: None,
        }
    }
}

impl From<(&IpAddr, &u16)> for SocketAddr {
    fn from((addr, port): (&IpAddr, &u16)) -> Self {
        SocketAddr::new(*addr, *port)
    }
}
impl From<(IpAddr, &u16)> for SocketAddr {
    fn from((addr, port): (IpAddr, &u16)) -> Self {
        (&addr, port).into()
    }
}
impl From<(&IpAddr, u16)> for SocketAddr {
    fn from((addr, port): (&IpAddr, u16)) -> Self {
        (addr, &port).into()
    }
}

impl From<(IpAddr, u16)> for SocketAddr {
    fn from((addr, port): (IpAddr, u16)) -> Self {
        (&addr, &port).into()
    }
}

impl From<(&IpAddr, &i32)> for SocketAddr {
    fn from((addr, port): (&IpAddr, &i32)) -> Self {
        SocketAddr::new(*addr, *port as u16)
    }
}
impl From<(IpAddr, &i32)> for SocketAddr {
    fn from((addr, port): (IpAddr, &i32)) -> Self {
        (&addr, port).into()
    }
}
impl From<(&IpAddr, i32)> for SocketAddr {
    fn from((addr, port): (&IpAddr, i32)) -> Self {
        (addr, &port).into()
    }
}

impl From<(IpAddr, i32)> for SocketAddr {
    fn from((addr, port): (IpAddr, i32)) -> Self {
        (&addr, &port).into()
    }
}

impl From<(&IpAddr, &usize)> for SocketAddr {
    fn from((addr, port): (&IpAddr, &usize)) -> Self {
        SocketAddr::new(*addr, *port as u16)
    }
}
impl From<(IpAddr, &usize)> for SocketAddr {
    fn from((addr, port): (IpAddr, &usize)) -> Self {
        (&addr, port).into()
    }
}
impl From<(&IpAddr, usize)> for SocketAddr {
    fn from((addr, port): (&IpAddr, usize)) -> Self {
        (addr, &port).into()
    }
}

impl From<(IpAddr, usize)> for SocketAddr {
    fn from((addr, port): (IpAddr, usize)) -> Self {
        (&addr, &port).into()
    }
}

// Struct representing multiple IP address and port bindings
#[derive(Debug, Clone, Default)]
pub struct SocketAddrs {
    pub bind_addr: Option<HashSet<SocketAddr>>,
    pub default_bind_addr: Option<HashSet<SocketAddr>>,
    pub default_port: Option<u16>,
}

impl SocketAddrs {
    pub fn with() -> SocketAddrsWithBuilder {
        SocketAddrsWithBuilder::default()
    }

    pub fn as_builder(self) -> SocketAddrsWithBuilder {
        SocketAddrsWithBuilder {
            bind_addr: self.bind_addr,
            default_bind_addr: self.default_bind_addr,
            default_port: self.default_port,
        }
    }

    pub fn add_bind_ipv4_port(&mut self, o1: u8, o2: u8, o3: u8, o4: u8, port: u16) {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(o1, o2, o3, o4)), port);
        self.add_bind_addr(addr);
    }

    pub fn add_bind_ipv6_port(
        &mut self,
        o1: u16,
        o2: u16,
        o3: u16,
        o4: u16,
        o5: u16,
        o6: u16,
        o7: u16,
        o8: u16,
        port: u16,
    ) {
        let addr = SocketAddr::new(
            IpAddr::V6(Ipv6Addr::new(o1, o2, o3, o4, o5, o6, o7, o8)),
            port,
        );
        self.add_bind_addr(addr);
    }

    pub fn add_bind_ipaddr_port(&mut self, ip: IpAddr, port: u16) {
        let addr = SocketAddr::new(ip, port);
        self.add_bind_addr(addr);
    }

    /// Adds a new bind address to the list.
    pub fn add_bind_addr(&mut self, addr: SocketAddr) {
        if let Some(ref mut bind_addrs) = self.bind_addr {
            bind_addrs.insert(addr);
        } else {
            let mut set = HashSet::new();
            set.insert(addr);
            self.bind_addr = Some(set);
        }
    }

    /// Adds a new default bind address to the list.
    pub fn add_default_bind_addr(&mut self, addr: SocketAddr) {
        if let Some(ref mut default_bind_addrs) = self.default_bind_addr {
            default_bind_addrs.insert(addr);
        } else {
            let mut set = HashSet::new();
            set.insert(addr);
            self.default_bind_addr = Some(set);
        }
    }

    pub fn with_default_port(mut self, port: u16) -> Self {
        self.default_port = Some(port);
        self
    }

    pub fn with_default_port_if_none(mut self, port: u16) -> Self {
        if self.default_port.is_none() {
            self.default_port = Some(port);
        }
        self
    }

    /// Creates a `SocketBindAddrs` with a single default address.
    pub fn with_default() -> Self {
        let default_addr = SocketAddr::default_addr();
        let mut set = HashSet::new();
        set.insert(default_addr);
        SocketAddrs {
            bind_addr: None,
            default_bind_addr: Some(set),
            default_port: None,
        }
    }

    /// Merges bind addresses with default bind addresses.
    /// Defaults are added only if no primary bind addresses are provided.
    pub fn merged_defaults(&mut self) -> Vec<SocketAddr> {
        if self.bind_addr.is_none() {
            self.bind_addr = self.default_bind_addr.take();
        }
        self.bind_addr
            .clone()
            .unwrap_or(HashSet::new())
            .into_iter()
            .collect::<Vec<SocketAddr>>()
    }
}

impl IntoIterator for SocketAddrs {
    type Item = SocketAddr;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(mut self) -> Self::IntoIter {
        self.merged_defaults().into_iter()
    }
}
trait TryWithSocketAddrs<T> {
    type Error;
    fn try_with_capture_ips(self) -> Result<T, Self::Error>;
    fn try_with_capture_ips_if_none(self) -> Result<T, Self::Error>;
}

impl TryWithSocketAddrs<SocketAddrs> for SocketAddrs {
    type Error = SocketAddrsError;
    fn try_with_capture_ips(mut self) -> Result<Self, Self::Error> {
        self.default_port
            .ok_or(SocketAddrsError::FailedToCaptureIp(
                "No default port".to_string(),
            ))
            .merge(
                IpAddrs::new()
                    .try_with_capture_ips()
                    .map_err(|e| SocketAddrsError::FailedToCaptureIp(format!("{:?}", e))),
                |port, ips| {
                    for ip in ips.into_iter() {
                        self.add_bind_ipaddr_port(ip, port);
                    }
                    Ok(self)
                },
            )
    }

    fn try_with_capture_ips_if_none(mut self) -> Result<Self, Self::Error> {
        if self.bind_addr.is_none() {
            self.try_with_capture_ips()
        } else {
            Ok(self)
        }
    }
}

impl StdToSocketAddrs for SocketAddrs {
    type Iter = std::vec::IntoIter<StdSocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        let mut addrs = Vec::new();

        if let Some(bind_addrs) = &self.bind_addr {
            for addr in bind_addrs {
                addrs.push(StdSocketAddr::from(addr.clone()));
            }
        }

        if let Some(default_bind_addrs) = &self.default_bind_addr {
            for addr in default_bind_addrs {
                addrs.push(StdSocketAddr::from(addr.clone()));
            }
        }

        Ok(addrs.into_iter())
    }
}

impl From<SocketAddrs> for Vec<SocketAddr> {
    fn from(bind_addrs: SocketAddrs) -> Self {
        let mut addrs = Vec::new();

        if let Some(bind_addrs) = &bind_addrs.bind_addr {
            for addr in bind_addrs {
                addrs.push(addr.clone());
            }
        }

        if let Some(default_bind_addrs) = &bind_addrs.default_bind_addr {
            for addr in default_bind_addrs {
                addrs.push(addr.clone());
            }
        }

        addrs
    }
}
impl From<SocketAddrs> for Vec<std::net::SocketAddr> {
    fn from(bind_addrs: SocketAddrs) -> Self {
        bind_addrs
            .to_socket_addrs()
            .map(|t| t.collect())
            .unwrap_or(Vec::new())
    }
}

impl From<(IpAddr, u16)> for SocketAddrs {
    fn from((ip, port): (IpAddr, u16)) -> Self {
        Self::from((&ip, &port))
    }
}
impl From<(&IpAddr, &u16)> for SocketAddrs {
    fn from(sock: (&IpAddr, &u16)) -> Self {
        let mut set = HashSet::new();
        set.insert(sock.into());
        SocketAddrs {
            bind_addr: Some(set),
            default_bind_addr: None,
            default_port: None,
        }
    }
}
impl From<(&IpAddr, &usize)> for SocketAddrs {
    fn from(sock: (&IpAddr, &usize)) -> Self {
        let mut set = HashSet::new();
        set.insert(sock.into());
        SocketAddrs {
            bind_addr: Some(set),
            default_bind_addr: None,
            default_port: None,
        }
    }
}
impl From<(IpAddr, usize)> for SocketAddrs {
    fn from(sock: (IpAddr, usize)) -> Self {
        let mut set = HashSet::new();
        set.insert(sock.into());
        SocketAddrs {
            bind_addr: Some(set),
            default_bind_addr: None,
            default_port: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SocketAddrsWithBuilder {
    bind_addr: Option<HashSet<SocketAddr>>,
    default_bind_addr: Option<HashSet<SocketAddr>>,
    default_port: Option<u16>,
}

impl SocketAddrsWithBuilder {}

pub trait SocketAddressTryWithBuilder<T> {
    type Error;

    fn ipv4_port(self, o1: u8, o2: u8, o3: u8, o4: u8, port: u16) -> Self;

    fn ipv6_port(
        self,
        o1: u16,
        o2: u16,
        o3: u16,
        o4: u16,
        o5: u16,
        o6: u16,
        o7: u16,
        o8: u16,
        port: u16,
    ) -> Self;

    fn ipaddr_port(self, ip: IpAddr, port: u16) -> Self;

    fn addr(self, addr: SocketAddr) -> Self;

    fn default_addr(self, addr: SocketAddr) -> Self;

    fn with_default_ipv4(self, o1: u8, o2: u8, o3: u8, o4: u8, port: u16) -> Self;

    fn with_default_ipv6(
        self,
        o1: u16,
        o2: u16,
        o3: u16,
        o4: u16,
        o5: u16,
        o6: u16,
        o7: u16,
        o8: u16,
        port: u16,
    ) -> Self;

    fn default_port(self, port: u16) -> Self;

    fn if_default_port(self, port: u16) -> Self;
    fn try_capture_ips(self) -> Result<T, Self::Error>;
    fn if_try_capture_ips(self) -> Result<SocketAddrsWithBuilder, Self::Error>;
    fn try_capture_broadcast(self) -> Result<T, Self::Error>;
    fn if_try_capture_broadcast(self) -> Result<SocketAddrsWithBuilder, Self::Error>;
    fn build(self) -> Result<SocketAddrs, Self::Error>;
}

impl SocketAddressTryWithBuilder<SocketAddrsWithBuilder> for SocketAddrsWithBuilder {
    type Error = SocketAddrsError;

    fn ipv4_port(mut self, o1: u8, o2: u8, o3: u8, o4: u8, port: u16) -> Self {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(o1, o2, o3, o4)), port);
        self.addr(addr)
    }

    fn ipv6_port(
        mut self,
        o1: u16,
        o2: u16,
        o3: u16,
        o4: u16,
        o5: u16,
        o6: u16,
        o7: u16,
        o8: u16,
        port: u16,
    ) -> Self {
        let addr = SocketAddr::new(
            IpAddr::V6(Ipv6Addr::new(o1, o2, o3, o4, o5, o6, o7, o8)),
            port,
        );
        self.addr(addr)
    }

    fn ipaddr_port(mut self, ip: IpAddr, port: u16) -> Self {
        let addr = SocketAddr::new(ip, port);
        self.addr(addr)
    }

    fn addr(mut self, addr: SocketAddr) -> Self {
        if let Some(ref mut bind_addrs) = self.bind_addr {
            bind_addrs.insert(addr);
        } else {
            let mut set = HashSet::new();
            set.insert(addr);
            self.bind_addr = Some(set);
        }
        self
    }

    fn default_addr(mut self, addr: SocketAddr) -> Self {
        if let Some(ref mut default_bind_addrs) = self.default_bind_addr {
            default_bind_addrs.insert(addr);
        } else {
            let mut set = HashSet::new();
            set.insert(addr);
            self.default_bind_addr = Some(set);
        }
        self
    }

    fn with_default_ipv4(mut self, o1: u8, o2: u8, o3: u8, o4: u8, port: u16) -> Self {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(o1, o2, o3, o4)), port);
        self.default_addr(addr)
    }

    fn with_default_ipv6(
        mut self,
        o1: u16,
        o2: u16,
        o3: u16,
        o4: u16,
        o5: u16,
        o6: u16,
        o7: u16,
        o8: u16,
        port: u16,
    ) -> Self {
        let addr = SocketAddr::new(
            IpAddr::V6(Ipv6Addr::new(o1, o2, o3, o4, o5, o6, o7, o8)),
            port,
        );
        self.default_addr(addr)
    }

    fn default_port(mut self, port: u16) -> Self {
        self.default_port = Some(port);
        self
    }

    fn if_default_port(mut self, port: u16) -> Self {
        if self.default_port.is_none() {
            self.default_port = Some(port);
        }
        self
    }
    fn try_capture_ips(mut self) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.default_port
            .ok_or(SocketAddrsError::FailedToCaptureIp(
                "No default port".to_string(),
            ))
            .merge(
                IpAddrs::new()
                    .try_with_capture_ips()
                    .map_err(|e| SocketAddrsError::FailedToCaptureIp(format!("{:?}", e))),
                |port, ips| {
                    for ip in ips.into_iter() {
                        self.bind_addr
                            .get_or_insert(HashSet::new())
                            .insert(SocketAddr::new(ip, port));
                    }
                    Ok(self)
                },
            )
    }
    fn if_try_capture_ips(mut self) -> Result<SocketAddrsWithBuilder, Self::Error> {
        if self.bind_addr.is_none() {
            self.try_capture_ips()
        } else {
            Ok(self)
        }
    }

    fn try_capture_broadcast(mut self) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.default_port
            .ok_or(SocketAddrsError::FailedToCaptureIp(
                "No default port".to_string(),
            ))
            .merge(
                IpAddrs::new()
                    .try_with_capture_broadcast()
                    .map_err(|e| SocketAddrsError::FailedToCaptureIp(format!("{:?}", e))),
                |port, ips| {
                    for ip in ips.into_iter() {
                        self.bind_addr
                            .get_or_insert(HashSet::new())
                            .insert(SocketAddr::new(ip, port));
                    }
                    Ok(self)
                },
            )
    }
    fn if_try_capture_broadcast(mut self) -> Result<SocketAddrsWithBuilder, Self::Error> {
        if self.bind_addr.is_none() {
            self.try_capture_broadcast()
        } else {
            Ok(self)
        }
    }
    fn build(self) -> Result<SocketAddrs, Self::Error> {
        // In this implementation, `build` is infallible because the configuration is
        // intentionally flexible and doesn't enforce constraints on the presence of addresses.
        Ok(SocketAddrs {
            bind_addr: self.bind_addr,
            default_bind_addr: self.default_bind_addr,
            default_port: self.default_port,
        })
    }
}

impl SocketAddressTryWithBuilder<SocketAddrsWithBuilder>
    for Result<SocketAddrsWithBuilder, SocketAddrsError>
{
    type Error = SocketAddrsError;

    fn ipv4_port(
        self,
        o1: u8,
        o2: u8,
        o3: u8,
        o4: u8,
        port: u16,
    ) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.map(|t| t.ipv4_port(o1, o2, o3, o4, port))
    }

    fn ipv6_port(
        self,
        o1: u16,
        o2: u16,
        o3: u16,
        o4: u16,
        o5: u16,
        o6: u16,
        o7: u16,
        o8: u16,
        port: u16,
    ) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.map(|t| t.ipv6_port(o1, o2, o3, o4, o5, o6, o7, o8, port))
    }

    fn ipaddr_port(self, ip: IpAddr, port: u16) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.map(|t| t.ipaddr_port(ip, port))
    }

    fn addr(self, addr: SocketAddr) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.map(|t| t.addr(addr))
    }

    fn default_addr(self, addr: SocketAddr) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.map(|t| t.default_addr(addr))
    }

    fn with_default_ipv4(
        self,
        o1: u8,
        o2: u8,
        o3: u8,
        o4: u8,
        port: u16,
    ) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.map(|t| t.with_default_ipv4(o1, o2, o3, o4, port))
    }

    fn with_default_ipv6(
        self,
        o1: u16,
        o2: u16,
        o3: u16,
        o4: u16,
        o5: u16,
        o6: u16,
        o7: u16,
        o8: u16,
        port: u16,
    ) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.map(|t| t.with_default_ipv6(o1, o2, o3, o4, o5, o6, o7, o8, port))
    }

    fn default_port(self, port: u16) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.map(|t| t.default_port(port))
    }

    fn if_default_port(self, port: u16) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.map(|t| t.if_default_port(port))
    }
    fn try_capture_ips(self) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.and_then(|t| t.try_capture_ips())
    }

    fn if_try_capture_ips(self) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.and_then(|t| t.if_try_capture_ips())
    }

    fn try_capture_broadcast(self) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.and_then(|t| t.try_capture_broadcast())
    }

    fn if_try_capture_broadcast(self) -> Result<SocketAddrsWithBuilder, Self::Error> {
        self.and_then(|t| t.if_try_capture_broadcast())
    }

    fn build(self) -> Result<SocketAddrs, Self::Error> {
        // In this implementation, `build` is infallible because the configuration is
        // intentionally flexible and doesn't enforce constraints on the presence of addresses.
        self.and_then(|t| t.build())
    }
}

#[derive(Debug)]
pub enum SocketAddrsError {
    /// Failed to capture IP addresses with additional context
    FailedToCaptureIp(String),
    /// Missing bind addresses in the SocketAddrs configuration
    MissingBindAddresses,
    /// Failed to determine broadcast addresses
    FailedToDetermineBroadcast(String),
}

impl std::fmt::Display for SocketAddrsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SocketAddrsError::FailedToCaptureIp(reason) => {
                write!(f, "Failed to capture IP addresses: {}", reason)
            }
            SocketAddrsError::MissingBindAddresses => {
                write!(f, "No bind addresses provided in the configuration")
            }
            SocketAddrsError::FailedToDetermineBroadcast(reason) => {
                write!(f, "Failed to determine broadcast addresses: {}", reason)
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr as StdSocketAddr};

    #[test]
    fn test_socket_bind_addr_new() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        assert_eq!(addr.ip(), Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))));
        assert_eq!(addr.port(), Some(8080));
    }

    #[test]
    fn test_socket_bind_addr_default() {
        let addr = SocketAddr::default_addr();
        assert_eq!(addr.ip(), Some(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))));
        assert_eq!(addr.port(), Some(9999));
    }

    #[test]
    fn test_socket_bind_addr_with_ip() {
        let addr =
            SocketAddr::default_addr().with_ip(Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))));
        assert_eq!(addr.ip(), Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))));
    }

    #[test]
    fn test_socket_bind_addr_with_port() {
        let addr = SocketAddr::default_addr().with_port(Some(8080));
        assert_eq!(addr.port(), Some(8080));
    }

    #[test]
    fn test_socket_bind_addr_to_socket_addrs() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let mut addrs_iter = addr.to_socket_addrs().unwrap();
        assert_eq!(
            addrs_iter.next().unwrap(),
            StdSocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
        );
    }

    #[test]
    fn test_from_socket_bind_addr_to_socket_addr() {
        let bind_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), 8080);
        let socket_addr: StdSocketAddr = bind_addr.into();
        assert_eq!(
            socket_addr,
            StdSocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), 8080)
        );
    }

    #[test]
    fn test_socket_bind_addrs_add_bind_ipv4_port() {
        let mut bind_addrs = SocketAddrs::default();
        bind_addrs.add_bind_ipv4_port(192, 168, 1, 1, 8080);
        assert!(bind_addrs.bind_addr.is_some());
    }

    #[test]
    fn test_socket_bind_addrs_add_bind_ipv6_port() {
        let mut bind_addrs = SocketAddrs::default();
        bind_addrs.add_bind_ipv6_port(0xfe80, 0, 0, 0, 0, 0, 0, 1, 8080);
        assert!(bind_addrs.bind_addr.is_some());
    }

    #[test]
    fn test_socket_bind_addrs_with_default() {
        let bind_addrs = SocketAddrs::with_default();
        assert!(bind_addrs.default_bind_addr.is_some());
    }

    #[test]
    fn test_socket_bind_addrs_merge_defaults() {
        let mut bind_addrs = SocketAddrs::with_default();
        bind_addrs.merged_defaults();
        assert!(bind_addrs.bind_addr.is_some());
    }

    #[test]
    fn test_to_socket_addrs_for_socket_bind_addrs() {
        let mut bind_addrs = SocketAddrs::default();
        bind_addrs.add_bind_ipv4_port(127, 0, 0, 1, 8080);
        let addrs_iter = bind_addrs.to_socket_addrs().unwrap();
        let addrs: Vec<StdSocketAddr> = addrs_iter.collect();
        assert_eq!(addrs.len(), 1);
        assert_eq!(
            addrs[0],
            StdSocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
        );
    }

    #[test]
    fn test_from_socket_bind_addrs_to_vec() {
        let mut bind_addrs = SocketAddrs::default();
        bind_addrs.add_bind_ipv4_port(127, 0, 0, 1, 8080);
        let vec: Vec<SocketAddr> = bind_addrs.into();
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn test_socket_bind_addr_as_broadcast_ipv4() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10)), 8080);
        let broadcast = addr
            .as_broadcast_addr(Some(IpAddr::V4(Ipv4Addr::new(255, 255, 255, 0))))
            .ok();
        assert_eq!(
            broadcast.unwrap().ip(),
            Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 255)))
        );
    }

    #[test]
    fn test_socket_bind_addr_as_broadcast_ipv6() {
        let addr = SocketAddr::new(
            IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1)),
            8080,
        );
        let broadcast = addr.as_broadcast_addr(Some(IpAddr::V6(Ipv6Addr::new(
            0xffff, 0xffff, 0xffff, 0xffff, 0, 0, 0, 0,
        ))));
        assert_eq!(
            broadcast.unwrap().ip(),
            Some(IpAddr::V6(Ipv6Addr::new(
                0x2001, 0xdb8, 0, 0, 0xffff, 0xffff, 0xffff, 0xffff
            )))
        );
    }
}
