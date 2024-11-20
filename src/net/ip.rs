use std::io::Error;
use std::net::IpAddr;
use if_addrs::get_if_addrs;
use crate::rails::ext::blocking::Tap;

pub struct IpAddrs(Vec<IpAddr>);

impl IpAddrs {
    pub fn new() -> Self {
        IpAddrs(Vec::new())
    }

    pub fn try_with_capture_ips(mut self) -> Result<Self, Error> {
        get_if_addrs()
            .map(|t| t.into_iter().map(|t| t.addr).map(|t| t.ip()).collect::<Vec<IpAddr>>())
            .tap_mut(|t| self.0.append(t))
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

fn get_brd_addr() -> {

}