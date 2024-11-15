use crate::externs::collections::HashSet;
use crate::rails::ext::blocking::Tap;
use core::fmt::{Display, Formatter};
#[cfg(feature = "regex")]
use regex::Regex;
use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, ToSocketAddrs};

// Struct representing a single IP address and port binding
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct SocketBindAddr {
    ip: Option<IpAddr>,
    port: Option<u16>,
    default_ip: Option<IpAddr>,
    default_port: Option<u16>,
}

impl SocketBindAddr {
    /// Creates a new `SocketBindAddr` with the provided IP and port.
    pub fn new(ip: IpAddr, port: u16) -> Self {
        SocketBindAddr {
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
        SocketBindAddr {
            ip: Some(ip),
            port: Some(port),
            default_ip: Some(default_ip),
            default_port: Some(default_port),
        }
    }
    pub fn as_broadcast_addr(&self, subnet_mask: Option<IpAddr>) -> Result<SocketBindAddr, String> {
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
impl SocketBindAddr {
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

impl Default for SocketBindAddr {
    fn default() -> Self {
        SocketBindAddr::default_addr()
    }
}

impl Display for SocketBindAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SocketBindAddr({}:{})",
            self.ip_with_defaults().to_string(),
            self.port_with_defaults()
        )
    }
}

impl ToSocketAddrs for SocketBindAddr {
    type Iter = std::vec::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        Ok(vec![SocketAddr::new(
            self.ip_with_defaults(),
            self.port_with_defaults(),
        )]
        .into_iter())
    }
}

impl From<SocketBindAddr> for SocketAddr {
    fn from(bind_addr: SocketBindAddr) -> Self {
        SocketAddr::new(bind_addr.ip_with_defaults(), bind_addr.port_with_defaults())
    }
}

impl From<&SocketBindAddr> for SocketAddr {
    fn from(bind_addr: &SocketBindAddr) -> Self {
        SocketAddr::new(bind_addr.ip_with_defaults(), bind_addr.port_with_defaults())
    }
}

impl From<SocketBindAddr> for SocketBindAddrs {
    fn from(bind_addr: SocketBindAddr) -> Self {
        let mut set = HashSet::new();
        set.insert(bind_addr);
        SocketBindAddrs {
            bind_addr: Some(set),
            default_bind_addr: None,
        }
    }
}

impl From<(&IpAddr, &u16)> for SocketBindAddr {
    fn from((addr, port): (&IpAddr, &u16)) -> Self {
        SocketBindAddr::new(*addr, *port)
    }
}
impl From<(IpAddr, &u16)> for SocketBindAddr {
    fn from((addr, port): (IpAddr, &u16)) -> Self {
        (&addr, port).into()
    }
}
impl From<(&IpAddr, u16)> for SocketBindAddr {
    fn from((addr, port): (&IpAddr, u16)) -> Self {
        (addr, &port).into()
    }
}

impl From<(IpAddr, u16)> for SocketBindAddr {
    fn from((addr, port): (IpAddr, u16)) -> Self {
        (&addr, &port).into()
    }
}

impl From<(&IpAddr, &i32)> for SocketBindAddr {
    fn from((addr, port): (&IpAddr, &i32)) -> Self {
        SocketBindAddr::new(*addr, *port as u16)
    }
}
impl From<(IpAddr, &i32)> for SocketBindAddr {
    fn from((addr, port): (IpAddr, &i32)) -> Self {
        (&addr, port).into()
    }
}
impl From<(&IpAddr, i32)> for SocketBindAddr {
    fn from((addr, port): (&IpAddr, i32)) -> Self {
        (addr, &port).into()
    }
}

impl From<(IpAddr, i32)> for SocketBindAddr {
    fn from((addr, port): (IpAddr, i32)) -> Self {
        (&addr, &port).into()
    }
}

impl From<(&IpAddr, &usize)> for SocketBindAddr {
    fn from((addr, port): (&IpAddr, &usize)) -> Self {
        SocketBindAddr::new(*addr, *port as u16)
    }
}
impl From<(IpAddr, &usize)> for SocketBindAddr {
    fn from((addr, port): (IpAddr, &usize)) -> Self {
        (&addr, port).into()
    }
}
impl From<(&IpAddr, usize)> for SocketBindAddr {
    fn from((addr, port): (&IpAddr, usize)) -> Self {
        (addr, &port).into()
    }
}

impl From<(IpAddr, usize)> for SocketBindAddr {
    fn from((addr, port): (IpAddr, usize)) -> Self {
        (&addr, &port).into()
    }
}

// Struct representing multiple IP address and port bindings
#[derive(Debug, Default)]
pub struct SocketBindAddrs {
    pub bind_addr: Option<HashSet<SocketBindAddr>>,
    pub default_bind_addr: Option<HashSet<SocketBindAddr>>,
}

impl SocketBindAddrs {
    pub fn add_bind_ipv4_port(&mut self, o1: u8, o2: u8, o3: u8, o4: u8, port: u16) {
        let addr = SocketBindAddr::new(IpAddr::V4(Ipv4Addr::new(o1, o2, o3, o4)), port);
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
        let addr = SocketBindAddr::new(
            IpAddr::V6(Ipv6Addr::new(o1, o2, o3, o4, o5, o6, o7, o8)),
            port,
        );
        self.add_bind_addr(addr);
    }

    pub fn add_bind_ipaddr_port(&mut self, ip: IpAddr, port: u16) {
        let addr = SocketBindAddr::new(ip, port);
        self.add_bind_addr(addr);
    }

    /// Adds a new bind address to the list.
    pub fn add_bind_addr(&mut self, addr: SocketBindAddr) {
        if let Some(ref mut bind_addrs) = self.bind_addr {
            bind_addrs.insert(addr);
        } else {
            let mut set = HashSet::new();
            set.insert(addr);
            self.bind_addr = Some(set);
        }
    }

    /// Adds a new default bind address to the list.
    pub fn add_default_bind_addr(&mut self, addr: SocketBindAddr) {
        if let Some(ref mut default_bind_addrs) = self.default_bind_addr {
            default_bind_addrs.insert(addr);
        } else {
            let mut set = HashSet::new();
            set.insert(addr);
            self.default_bind_addr = Some(set);
        }
    }

    /// Creates a `SocketBindAddrs` with a single default address.
    pub fn with_default() -> Self {
        let default_addr = SocketBindAddr::default_addr();
        let mut set = HashSet::new();
        set.insert(default_addr);
        SocketBindAddrs {
            bind_addr: None,
            default_bind_addr: Some(set),
        }
    }

    /// Merges bind addresses with default bind addresses.
    /// Defaults are added only if no primary bind addresses are provided.
    pub fn merge_defaults(&mut self) {
        if self.bind_addr.is_none() {
            self.bind_addr = self.default_bind_addr.take();
        }
    }
}

impl ToSocketAddrs for SocketBindAddrs {
    type Iter = std::vec::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        let mut addrs = Vec::new();

        if let Some(bind_addrs) = &self.bind_addr {
            for addr in bind_addrs {
                addrs.push(SocketAddr::from(addr.clone()));
            }
        }

        if let Some(default_bind_addrs) = &self.default_bind_addr {
            for addr in default_bind_addrs {
                addrs.push(SocketAddr::from(addr.clone()));
            }
        }

        Ok(addrs.into_iter())
    }
}

impl From<SocketBindAddrs> for Vec<SocketBindAddr> {
    fn from(bind_addrs: SocketBindAddrs) -> Self {
        if let Some(addrs) = bind_addrs.bind_addr {
            addrs.iter().cloned().collect()
        } else {
            Vec::new()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

    #[test]
    fn test_socket_bind_addr_new() {
        let addr = SocketBindAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        assert_eq!(addr.ip(), Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))));
        assert_eq!(addr.port(), Some(8080));
    }

    #[test]
    fn test_socket_bind_addr_default() {
        let addr = SocketBindAddr::default_addr();
        assert_eq!(addr.ip(), Some(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))));
        assert_eq!(addr.port(), Some(9999));
    }

    #[test]
    fn test_socket_bind_addr_with_ip() {
        let addr =
            SocketBindAddr::default_addr().with_ip(Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))));
        assert_eq!(addr.ip(), Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))));
    }

    #[test]
    fn test_socket_bind_addr_with_port() {
        let addr = SocketBindAddr::default_addr().with_port(Some(8080));
        assert_eq!(addr.port(), Some(8080));
    }

    #[test]
    fn test_socket_bind_addr_to_socket_addrs() {
        let addr = SocketBindAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let mut addrs_iter = addr.to_socket_addrs().unwrap();
        assert_eq!(
            addrs_iter.next().unwrap(),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
        );
    }

    #[test]
    fn test_from_socket_bind_addr_to_socket_addr() {
        let bind_addr = SocketBindAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), 8080);
        let socket_addr: SocketAddr = bind_addr.into();
        assert_eq!(
            socket_addr,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), 8080)
        );
    }

    #[test]
    fn test_socket_bind_addrs_add_bind_ipv4_port() {
        let mut bind_addrs = SocketBindAddrs::default();
        bind_addrs.add_bind_ipv4_port(192, 168, 1, 1, 8080);
        assert!(bind_addrs.bind_addr.is_some());
    }

    #[test]
    fn test_socket_bind_addrs_add_bind_ipv6_port() {
        let mut bind_addrs = SocketBindAddrs::default();
        bind_addrs.add_bind_ipv6_port(0xfe80, 0, 0, 0, 0, 0, 0, 1, 8080);
        assert!(bind_addrs.bind_addr.is_some());
    }

    #[test]
    fn test_socket_bind_addrs_with_default() {
        let bind_addrs = SocketBindAddrs::with_default();
        assert!(bind_addrs.default_bind_addr.is_some());
    }

    #[test]
    fn test_socket_bind_addrs_merge_defaults() {
        let mut bind_addrs = SocketBindAddrs::with_default();
        bind_addrs.merge_defaults();
        assert!(bind_addrs.bind_addr.is_some());
    }

    #[test]
    fn test_to_socket_addrs_for_socket_bind_addrs() {
        let mut bind_addrs = SocketBindAddrs::default();
        bind_addrs.add_bind_ipv4_port(127, 0, 0, 1, 8080);
        let addrs_iter = bind_addrs.to_socket_addrs().unwrap();
        let addrs: Vec<SocketAddr> = addrs_iter.collect();
        assert_eq!(addrs.len(), 1);
        assert_eq!(
            addrs[0],
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
        );
    }

    #[test]
    fn test_from_socket_bind_addrs_to_vec() {
        let mut bind_addrs = SocketBindAddrs::default();
        bind_addrs.add_bind_ipv4_port(127, 0, 0, 1, 8080);
        let vec: Vec<SocketBindAddr> = bind_addrs.into();
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn test_socket_bind_addr_as_broadcast_ipv4() {
        let addr = SocketBindAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10)), 8080);
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
        let addr = SocketBindAddr::new(
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
