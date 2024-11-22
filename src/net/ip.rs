use crate::rails::ext::blocking::Tap;
use alloc::vec::Vec;
use if_addrs::get_if_addrs;
use std::io::Error;
use std::net::IpAddr;

pub struct IpAddrs(Vec<IpAddr>);

impl IpAddrs {
    pub fn new() -> Self {
        IpAddrs(Vec::new())
    }

    pub fn try_with_capture_ips(mut self) -> Result<Self, Error> {
        get_if_addrs()
            .map(|t| {
                t.into_iter()
                    .map(|t| t.addr)
                    .map(|t| t.ip())
                    .collect::<Vec<IpAddr>>()
            })
            .map(|mut t| {
                self.0.append(&mut t);
                self
            })
    }

    pub fn try_with_capture_broadcast(mut self) -> Result<Self, Error> {
        get_if_addrs()
            .map(|t| {
                t.into_iter()
                    .map(|t| t.addr)
                    .map(|t| match t {
                        if_addrs::IfAddr::V4(t) => t.broadcast.map(|t| IpAddr::V4(t)),
                        if_addrs::IfAddr::V6(t) => t.broadcast.map(|t| IpAddr::V6(t)),
                    })
                    .flatten()
                    .collect::<Vec<IpAddr>>()
            })
            .map(|mut t| {
                self.0.append(&mut t);
                self
            })
    }

    pub fn with_all_ips(mut self, ips: Vec<IpAddr>) -> Self {
        self.0 = ips;
        self
    }
}

impl From<Vec<IpAddr>> for IpAddrs {
    fn from(ips: Vec<IpAddr>) -> Self {
        IpAddrs(ips)
    }
}
