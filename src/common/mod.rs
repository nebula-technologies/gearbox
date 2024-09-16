pub mod boxed_future;
pub mod process;
pub mod try_default;

pub use boxed_future::BoxedFuture;
use core::net::IpAddr;
use if_addrs::get_if_addrs;
pub use try_default::TryDefault;

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
