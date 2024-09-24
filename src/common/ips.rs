use if_addrs::get_if_addrs;
use std::net::IpAddr;

pub fn get_ips() -> Vec<IpAddr> {
    #[cfg(not(feature = "std"))]
    {
        Vec::new()
    }

    #[cfg(feature = "std")]
    {
        let mut ip_addresses = Vec::new();

        if let Ok(interfaces) = get_if_addrs() {
            for iface in interfaces {
                if !iface.is_loopback() {
                    ip_addresses.push(iface.ip());
                }
            }
        }

        ip_addresses
    }
}
