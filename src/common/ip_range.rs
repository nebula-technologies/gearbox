use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use core::num::ParseIntError;
use core::str::FromStr;

pub enum RangeParseError {
    InvalidRange(String),
    InvalidPart(String),
    InvalidMultiRange(String),
    InvalidExact(String),
    IntegerParsingError(ParseIntError),
}

impl From<ParseIntError> for RangeParseError {
    fn from(e: ParseIntError) -> Self {
        RangeParseError::IntegerParsingError(e)
    }
}

#[derive(Debug, Clone)]
enum Range {
    Exact(u16),
    Range(u16, u16),
    MultiRange(Vec<Range>),
    Wildcard,
}

impl Range {
    fn contains(&self, value: u16) -> bool {
        match self {
            Range::Exact(num) => *num == value,
            Range::Range(start, end) => *start <= value && value <= *end,
            Range::MultiRange(ranges) => ranges.iter().any(|r| r.contains(value)),
            Range::Wildcard => true,
        }
    }
}

impl FromStr for Range {
    type Err = RangeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "*" {
            Ok(Range::Wildcard)
        } else if s.contains('-') || s.contains(',') {
            let list = s.split(',');
            list.map(|s| {
                if s.contains('-') {
                    let parts: Vec<&str> = s.split('-').collect();
                    let start = if parts[0].is_empty() {
                        0
                    } else {
                        parts[0].parse::<u16>()?
                    };
                    let end = if parts[1].is_empty() {
                        255
                    } else {
                        parts[1].parse::<u16>()?
                    };
                    Ok(Range::Range(start, end))
                } else {
                    Ok(Range::Exact(s.parse::<u16>()?))
                }
            })
            .collect::<Result<Vec<Range>, RangeParseError>>()
            .map(Range::MultiRange)
        } else {
            s.parse::<u16>()
                .map_err(|_| RangeParseError::InvalidExact(s.to_string()))
                .map(Range::Exact)
        }
    }
}

#[derive(Debug, Clone)]
struct Ipv4Range {
    part_1: Range,
    part_2: Range,
    part_3: Range,
    part_4: Range,
}

impl Ipv4Range {
    fn contains(&self, ip: &Ipv4Addr) -> bool {
        let octets = ip.octets();
        self.part_1.contains(octets[0] as u16)
            && self.part_2.contains(octets[1] as u16)
            && self.part_3.contains(octets[2] as u16)
            && self.part_4.contains(octets[3] as u16)
    }
}

#[derive(Debug, Clone)]
struct Ipv6Range {
    part_1: Range,
    part_2: Range,
    part_3: Range,
    part_4: Range,
    part_5: Range,
    part_6: Range,
    part_7: Range,
    part_8: Range,
}

impl Ipv6Range {
    fn contains(&self, ip: &Ipv6Addr) -> bool {
        let segments = ip.segments();
        self.part_1.contains(segments[0])
            && self.part_2.contains(segments[1])
            && self.part_3.contains(segments[2])
            && self.part_4.contains(segments[3])
            && self.part_5.contains(segments[4])
            && self.part_6.contains(segments[5])
            && self.part_7.contains(segments[6])
            && self.part_8.contains(segments[7])
    }
}

#[derive(Debug, Clone)]
pub struct IpRanges(Vec<Ipv4Range>, Vec<Ipv6Range>);

impl IpRanges {
    pub fn contains(&self, ip: &IpAddr) -> bool {
        match ip {
            IpAddr::V4(ipv4) => self.0.iter().any(|range| range.contains(ipv4)),
            IpAddr::V6(ipv6) => self.1.iter().any(|range| range.contains(ipv6)),
        }
    }

    pub fn add_ipv4_range(
        &mut self,
        p1: &str,
        p2: &str,
        p3: &str,
        p4: &str,
    ) -> Result<(), Ipv4RangeError> {
        self.0.push(Ipv4Range {
            part_1: Range::from_str(p1)?,
            part_2: Range::from_str(p2)?,
            part_3: Range::from_str(p3)?,
            part_4: Range::from_str(p4)?,
        });
        Ok(())
    }

    pub fn add_ipv6_range(
        &mut self,
        p1: &str,
        p2: &str,
        p3: &str,
        p4: &str,
        p5: &str,
        p6: &str,
        p7: &str,
        p8: &str,
    ) -> Result<(), Ipv4RangeError> {
        self.1.push(Ipv6Range {
            part_1: Range::from_str(p1)?,
            part_2: Range::from_str(p2)?,
            part_3: Range::from_str(p3)?,
            part_4: Range::from_str(p4)?,
            part_5: Range::from_str(p5)?,
            part_6: Range::from_str(p6)?,
            part_7: Range::from_str(p7)?,
            part_8: Range::from_str(p8)?,
        });
        Ok(())
    }

    pub fn new_ipv4_range(
        p1: &str,
        p2: &str,
        p3: &str,
        p4: &str,
    ) -> Result<IpRanges, Ipv4RangeError> {
        let mut slf = Self::default();
        slf.add_ipv4_range(p1, p2, p3, p4)?;
        Ok(slf)
    }

    pub fn new_ipv6_range(
        p1: &str,
        p2: &str,
        p3: &str,
        p4: &str,
        p5: &str,
        p6: &str,
        p7: &str,
        p8: &str,
    ) -> Result<IpRanges, Ipv4RangeError> {
        let mut slf = Self::default();
        slf.add_ipv6_range(p1, p2, p3, p4, p5, p6, p7, p8)?;
        Ok(slf)
    }

    pub fn add_ipv4_cidr_range(
        &mut self,
        ip: Ipv4Addr,
        prefix_length: u8,
    ) -> Result<(), Ipv4RangeError> {
        let (start_ip, end_ip) = Self::ipv4_cidr_to_range(ip, prefix_length);
        let start = start_ip.octets();
        let end = end_ip.octets();

        self.0.push(Ipv4Range {
            part_1: Range::Range(start[0] as u16, end[0] as u16),
            part_2: Range::Range(start[1] as u16, end[1] as u16),
            part_3: Range::Range(start[2] as u16, end[2] as u16),
            part_4: Range::Range(start[3] as u16, end[3] as u16),
        });
        Ok(())
    }

    pub fn new_ipv4_cidr_range(
        ip: Ipv4Addr,
        prefix_length: u8,
    ) -> Result<IpRanges, Ipv4RangeError> {
        let mut slf = Self::default();
        slf.add_ipv4_cidr_range(ip, prefix_length)?;
        Ok(slf)
    }

    pub fn add_ipv6_cidr_range(
        &mut self,
        ip: Ipv6Addr,
        prefix_length: u8,
    ) -> Result<(), Ipv4RangeError> {
        let (start_ip, end_ip) = Self::ipv6_cidr_to_range(ip, prefix_length);
        let start = start_ip.segments();
        let end = end_ip.segments();

        self.1.push(Ipv6Range {
            part_1: Range::Range(start[0], end[0]),
            part_2: Range::Range(start[1], end[1]),
            part_3: Range::Range(start[2], end[2]),
            part_4: Range::Range(start[3], end[3]),
            part_5: Range::Range(start[4], end[4]),
            part_6: Range::Range(start[5], end[5]),
            part_7: Range::Range(start[6], end[6]),
            part_8: Range::Range(start[7], end[7]),
        });
        Ok(())
    }

    pub fn new_ipv6_cidr_range(
        ip: Ipv6Addr,
        prefix_length: u8,
    ) -> Result<IpRanges, Ipv4RangeError> {
        let mut slf = Self::default();
        slf.add_ipv6_cidr_range(ip, prefix_length)?;
        Ok(slf)
    }

    // A helper to calculate the network range for IPv4 CIDR notation
    fn ipv4_cidr_to_range(ip: Ipv4Addr, prefix_length: u8) -> (Ipv4Addr, Ipv4Addr) {
        let ip_u32 = u32::from(ip);
        let mask = (!0u32).checked_shl(32 - prefix_length as u32).unwrap_or(0);
        let network = ip_u32 & mask;
        let broadcast = network | !mask;

        (Ipv4Addr::from(network), Ipv4Addr::from(broadcast))
    }

    // A helper to calculate the network range for IPv6 CIDR notation
    fn ipv6_cidr_to_range(ip: Ipv6Addr, prefix_length: u8) -> (Ipv6Addr, Ipv6Addr) {
        let ip_u128 = u128::from(ip);
        let mask = (!0u128)
            .checked_shl(128 - prefix_length as u32)
            .unwrap_or(0);
        let network = ip_u128 & mask;
        let broadcast = network | !mask;

        (Ipv6Addr::from(network), Ipv6Addr::from(broadcast))
    }
}

impl Default for IpRanges {
    fn default() -> Self {
        IpRanges(Vec::new(), Vec::new())
    }
}

#[derive(Debug)]
pub enum IpRange {
    Ipv4(Ipv4Range),
    Ipv6(Ipv6Range),
}

impl IpRange {
    pub fn contains(&self, ip: &IpAddr) -> bool {
        match (self, ip) {
            (IpRange::Ipv4(range), IpAddr::V4(ipv4)) => range.contains(ipv4),
            (IpRange::Ipv6(range), IpAddr::V6(ipv6)) => range.contains(ipv6),
            _ => false, // Mismatch of IP version
        }
    }

    pub fn new_ipv4_range(
        p1: &str,
        p2: &str,
        p3: &str,
        p4: &str,
    ) -> Result<IpRange, Ipv4RangeError> {
        Ok(Self::Ipv4(Ipv4Range {
            part_1: Range::from_str(p1)?,
            part_2: Range::from_str(p2)?,
            part_3: Range::from_str(p3)?,
            part_4: Range::from_str(p4)?,
        }))
    }

    pub fn new_ipv6_range(
        p1: &str,
        p2: &str,
        p3: &str,
        p4: &str,
        p5: &str,
        p6: &str,
        p7: &str,
        p8: &str,
    ) -> Result<IpRange, Ipv4RangeError> {
        Ok(Self::Ipv6(Ipv6Range {
            part_1: Range::from_str(p1)?,
            part_2: Range::from_str(p2)?,
            part_3: Range::from_str(p3)?,
            part_4: Range::from_str(p4)?,
            part_5: Range::from_str(p5)?,
            part_6: Range::from_str(p6)?,
            part_7: Range::from_str(p7)?,
            part_8: Range::from_str(p8)?,
        }))
    }

    pub fn new_ipv4_cidr_range(ip: Ipv4Addr, prefix_length: u8) -> Result<IpRange, Ipv4RangeError> {
        let (start_ip, end_ip) = Self::ipv4_cidr_to_range(ip, prefix_length);
        let start = start_ip.octets();
        let end = end_ip.octets();

        Ok(Self::Ipv4(Ipv4Range {
            part_1: Range::Range(start[0] as u16, end[0] as u16),
            part_2: Range::Range(start[1] as u16, end[1] as u16),
            part_3: Range::Range(start[2] as u16, end[2] as u16),
            part_4: Range::Range(start[3] as u16, end[3] as u16),
        }))
    }

    pub fn new_ipv6_cidr_range(ip: Ipv6Addr, prefix_length: u8) -> Result<IpRange, Ipv4RangeError> {
        let (start_ip, end_ip) = Self::ipv6_cidr_to_range(ip, prefix_length);
        let start = start_ip.segments();
        let end = end_ip.segments();

        Ok(Self::Ipv6(Ipv6Range {
            part_1: Range::Range(start[0], end[0]),
            part_2: Range::Range(start[1], end[1]),
            part_3: Range::Range(start[2], end[2]),
            part_4: Range::Range(start[3], end[3]),
            part_5: Range::Range(start[4], end[4]),
            part_6: Range::Range(start[5], end[5]),
            part_7: Range::Range(start[6], end[6]),
            part_8: Range::Range(start[7], end[7]),
        }))
    }

    // A helper to calculate the network range for IPv4 CIDR notation
    fn ipv4_cidr_to_range(ip: Ipv4Addr, prefix_length: u8) -> (Ipv4Addr, Ipv4Addr) {
        let ip_u32 = u32::from(ip);
        let mask = (!0u32).checked_shl(32 - prefix_length as u32).unwrap_or(0);
        let network = ip_u32 & mask;
        let broadcast = network | !mask;

        (Ipv4Addr::from(network), Ipv4Addr::from(broadcast))
    }

    // A helper to calculate the network range for IPv6 CIDR notation
    fn ipv6_cidr_to_range(ip: Ipv6Addr, prefix_length: u8) -> (Ipv6Addr, Ipv6Addr) {
        let ip_u128 = u128::from(ip);
        let mask = (!0u128)
            .checked_shl(128 - prefix_length as u32)
            .unwrap_or(0);
        let network = ip_u128 & mask;
        let broadcast = network | !mask;

        (Ipv6Addr::from(network), Ipv6Addr::from(broadcast))
    }
}

pub enum Ipv4RangeError {
    PartParsingError(RangeParseError),
}

impl From<RangeParseError> for Ipv4RangeError {
    fn from(e: RangeParseError) -> Self {
        Ipv4RangeError::PartParsingError(e)
    }
}
