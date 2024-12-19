use super::SocketTryWithBuilder;
use crate::net::socket_addr::socket_addr_with_builder::SocketAddrWithBuilder;
use crate::net::socket_addr::SocketAddrs;
use core::fmt;
use core::fmt::{Display, Formatter};
use core::marker::PhantomData;
#[cfg(feature = "regex")]
use regex::Regex;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io::Result as IoResult;
use std::net::{
    IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr as StdSocketAddr, ToSocketAddrs as StdToSocketAddrs,
};

pub struct SocketAddr<O = SocketAddrWithBuilder, B = SocketAddrWithBuilder>
where
    B: SocketTryWithBuilder<O>,
{
    ip: Option<IpAddr>,
    port: Option<u16>,
    default_ip: Option<IpAddr>,
    default_port: Option<u16>,
    phantom: PhantomData<(O, B)>,
}

impl SocketAddr {
    /// Creates a new `SocketBindAddr` with the provided IP and port.
    pub fn new(ip: IpAddr, port: u16) -> Self {
        SocketAddr {
            ip: Some(ip),
            port: Some(port),
            default_ip: None,
            default_port: None,
            phantom: Default::default(),
        }
    }

    // pub fn into_builder(self) -> SocketAddrWithBuilder {}

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
            phantom: Default::default(),
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
            phantom: Default::default(),
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

impl Clone for SocketAddr {
    fn clone(&self) -> Self {
        SocketAddr {
            ip: self.ip,
            port: self.port,
            default_ip: self.default_ip,
            default_port: self.default_port,
            phantom: Default::default(),
        }
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

impl<O, B> fmt::Debug for SocketAddr<O, B>
where
    B: SocketTryWithBuilder<O>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SocketAddr")
            .field("ip", &self.ip)
            .field("port", &self.port)
            .field("default_ip", &self.default_ip)
            .field("default_port", &self.default_port)
            // Exclude `phantom`
            .finish()
    }
}

impl<O, B> Hash for SocketAddr<O, B>
where
    B: SocketTryWithBuilder<O>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ip.hash(state);
        self.port.hash(state);
        self.default_ip.hash(state);
        self.default_port.hash(state);
        // PhantomData is intentionally not hashed.
    }
}

impl<O, B> PartialEq for SocketAddr<O, B>
where
    B: SocketTryWithBuilder<O>,
{
    fn eq(&self, other: &Self) -> bool {
        self.ip == other.ip
            && self.port == other.port
            && self.default_ip == other.default_ip
            && self.default_port == other.default_port
    }
}

impl<O, B> Eq for SocketAddr<O, B> where B: SocketTryWithBuilder<O> {}

impl StdToSocketAddrs for SocketAddr {
    type Iter = std::vec::IntoIter<StdSocketAddr>;

    fn to_socket_addrs(&self) -> IoResult<Self::Iter> {
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
