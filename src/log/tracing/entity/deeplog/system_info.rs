use core::{net::IpAddr, option::Option};
use serde_derive::{Deserialize, Serialize};
use std::fs::read_to_string;
#[cfg(feature = "std")]
use std::process::Command;

#[cfg(feature = "std")]
use pnet::datalink;
use spin::RwLock;

pub static HOSTNAME: RwLock<Option<String>> = RwLock::new(None);
pub static MAC_ADDRESSES: RwLock<Vec<String>> = RwLock::new(Vec::new());
pub static OS_INFO: RwLock<Option<String>> = RwLock::new(None);
pub static KERNEL_VERSION: RwLock<Option<String>> = RwLock::new(None);
pub static SYSTEM_ARCHITECTURE: RwLock<Option<String>> = RwLock::new(None);
pub static VIRTUALIZATION: RwLock<Option<String>> = RwLock::new(None);
pub static BOOT_TIME: RwLock<Option<u64>> = RwLock::new(None);
pub static TOTAL_MEMORY_MB: RwLock<Option<u64>> = RwLock::new(None);
pub static DISK_TOTAL_GB: RwLock<Option<u64>> = RwLock::new(None);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemInfo {
    pub hostname: Option<String>,
    pub ip_addresses: Vec<String>,
    pub mac_addresses: Vec<String>,
    pub os_info: Option<String>,
    pub kernel_version: Option<String>,
    pub total_memory_mb: Option<u64>,
    pub available_memory_mb: Option<u64>,
    pub disk_total_gb: Option<u64>,
    pub disk_free_gb: Option<u64>,
    pub load_avg: Option<String>,
    pub uptime_secs: Option<u64>,
    pub system_architecture: Option<String>,
    pub virtualization: Option<String>,
    pub running_processes: Option<u64>,
    pub boot_time: Option<u64>,
    pub swap_total_mb: Option<u64>,
    pub swap_free_mb: Option<u64>,
    pub network_bandwidth: Vec<(String, u64)>,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            hostname: get_hostname(),
            ip_addresses: get_ip_addresses(),
            mac_addresses: get_mac_addresses(),
            os_info: get_os_info(),
            kernel_version: get_kernel_version(),
            total_memory_mb: get_total_memory_mb(),
            available_memory_mb: get_available_memory_mb(),
            disk_total_gb: get_disk_total(),
            disk_free_gb: get_disk_free(),
            load_avg: get_load_avg(),
            uptime_secs: get_uptime_secs(),
            system_architecture: get_system_architecture(),
            virtualization: get_virtualization(),
            running_processes: get_running_processes(),
            boot_time: get_boot_time(),
            swap_total_mb: get_swap_total_mb(),
            swap_free_mb: get_swap_free_mb(),
            network_bandwidth: get_all_network_interfaces(),
        }
    }
}

#[cfg(all(feature = "std", target_family = "unix"))]
fn run_command(cmd: &str, args: &[&str]) -> Option<String> {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                None
            }
        })
}

fn get_hostname() -> Option<String> {
    #[cfg(not(feature = "std"))]
    {
        None
    }

    #[cfg(feature = "std")]
    {
        let mut hostname = HOSTNAME.write();
        if hostname.is_none() {
            if let Ok(output) = Command::new("hostname").output() {
                *hostname = Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
            }
        }
        hostname.clone()
    }
}

fn get_mac_addresses() -> Vec<String> {
    #[cfg(not(feature = "std"))]
    {
        Vec::new()
    }

    #[cfg(feature = "std")]
    {
        let mut mac_addresses = MAC_ADDRESSES.write();
        if mac_addresses.is_empty() {
            if let Ok(output) = Command::new("ip").arg("link").output() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = output_str.lines().collect();
                for line in lines {
                    if line.contains("link/ether") {
                        let fields: Vec<&str> = line.split_whitespace().collect();
                        if fields.len() >= 2 {
                            mac_addresses.push(fields[1].to_string());
                        }
                    }
                }
            }
        }
        mac_addresses.clone()
    }
}

fn get_os_info() -> Option<String> {
    #[cfg(not(feature = "std"))]
    {
        None
    }

    #[cfg(feature = "std")]
    {
        let mut os_info = OS_INFO.write();
        if os_info.is_none() {
            if let Ok(output) = Command::new("uname").arg("-o").output() {
                *os_info = Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
            }
        }
        os_info.clone()
    }
}

fn get_kernel_version() -> Option<String> {
    #[cfg(not(feature = "std"))]
    {
        None
    }

    #[cfg(feature = "std")]
    {
        let mut kernel_version = KERNEL_VERSION.write();
        if kernel_version.is_none() {
            if let Ok(output) = Command::new("uname").arg("-r").output() {
                *kernel_version = Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
            }
        }
        kernel_version.clone()
    }
}

fn get_total_memory_mb() -> Option<u64> {
    #[cfg(not(feature = "std"))]
    {
        None
    }

    #[cfg(feature = "std")]
    {
        let mut total_memory_mb = TOTAL_MEMORY_MB.write();
        if total_memory_mb.is_none() {
            if let Ok(output) = Command::new("free").arg("-m").output() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = output_str.lines().collect();
                if lines.len() > 1 {
                    let fields: Vec<&str> = lines[1].split_whitespace().collect();
                    if fields.len() >= 2 {
                        *total_memory_mb = fields[1].parse::<u64>().ok();
                    }
                }
            }
        }
        total_memory_mb.clone()
    }
}

fn get_available_memory_mb() -> Option<u64> {
    #[cfg(not(feature = "std"))]
    {
        None
    }

    #[cfg(feature = "std")]
    {
        if let Ok(output) = Command::new("free").arg("-m").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = output_str.lines().collect();
            if lines.len() > 2 {
                let fields: Vec<&str> = lines[2].split_whitespace().collect();
                if fields.len() >= 6 {
                    return fields[6].parse::<u64>().ok();
                }
            }
        }
        None
    }
}

fn get_system_architecture() -> Option<String> {
    #[cfg(not(feature = "std"))]
    {
        None
    }

    #[cfg(feature = "std")]
    {
        let mut system_architecture = SYSTEM_ARCHITECTURE.write();
        if system_architecture.is_none() {
            if let Ok(output) = Command::new("uname").arg("-m").output() {
                *system_architecture =
                    Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
            }
        }
        system_architecture.clone()
    }
}

fn get_virtualization() -> Option<String> {
    #[cfg(not(feature = "std"))]
    {
        None
    }

    #[cfg(feature = "std")]
    {
        let mut virtualization = VIRTUALIZATION.write();
        if virtualization.is_none() {
            if let Ok(output) = Command::new("systemd-detect-virt").output() {
                *virtualization = Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
            }
        }
        virtualization.clone()
    }
}

fn get_load_avg() -> Option<String> {
    #[cfg(not(feature = "std"))]
    {
        None
    }

    #[cfg(feature = "std")]
    {
        if let Ok(output) = Command::new("uptime").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = output_str.split(',').collect();
            if parts.len() >= 3 {
                return Some(parts[2..].join(",").trim().to_string());
            }
        }
        None
    }
}

fn get_uptime_secs() -> Option<u64> {
    #[cfg(not(feature = "std"))]
    {
        None
    }

    #[cfg(feature = "std")]
    {
        if let Ok(output) = Command::new("cat").arg("/proc/uptime").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let fields: Vec<&str> = output_str.split_whitespace().collect();
            if let Some(first_field) = fields.get(0) {
                return first_field.parse::<f64>().ok().map(|v| v as u64);
            }
        }
        None
    }
}

fn get_boot_time() -> Option<u64> {
    #[cfg(not(feature = "std"))]
    {
        None
    }

    #[cfg(feature = "std")]
    {
        let mut boot_time = BOOT_TIME.write();
        if boot_time.is_none() {
            if let Some(uptime) = get_uptime_secs() {
                let boot_time_secs = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .ok()?
                    .as_secs()
                    - uptime;
                *boot_time = Some(boot_time_secs);
            }
        }
        boot_time.clone()
    }
}

fn get_running_processes() -> Option<u64> {
    #[cfg(not(feature = "std"))]
    {
        None
    }

    #[cfg(feature = "std")]
    {
        if let Ok(output) = Command::new("ps").arg("-e").arg("--no-headers").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            Some(output_str.lines().count() as u64)
        } else {
            None
        }
    }
}

fn get_swap_total_mb() -> Option<u64> {
    #[cfg(not(feature = "std"))]
    {
        None
    }

    #[cfg(feature = "std")]
    {
        if let Ok(output) = Command::new("free").arg("-m").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = output_str.lines().collect();
            if lines.len() > 3 {
                let fields: Vec<&str> = lines[3].split_whitespace().collect();
                if fields.len() >= 2 {
                    return fields[1].parse::<u64>().ok();
                }
            }
        }
        None
    }
}

fn get_swap_free_mb() -> Option<u64> {
    #[cfg(not(feature = "std"))]
    {
        None
    }

    #[cfg(feature = "std")]
    {
        if let Ok(output) = Command::new("free").arg("-m").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = output_str.lines().collect();
            if lines.len() > 3 {
                let fields: Vec<&str> = lines[3].split_whitespace().collect();
                if fields.len() >= 4 {
                    return fields[3].parse::<u64>().ok();
                }
            }
        }
        None
    }
}

fn get_ip_addresses() -> Vec<String> {
    #[cfg(not(feature = "std"))]
    {
        Vec::new()
    }

    #[cfg(feature = "std")]
    {
        let mut ip_addresses = Vec::new();
        if let Ok(output) = Command::new("ip").arg("addr").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.trim().starts_with("inet ") {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    if fields.len() >= 2 {
                        let ip = fields[1].split('/').next().unwrap().to_string(); // Extract IP part
                        ip_addresses.push(ip);
                    }
                }
            }
        }
        ip_addresses
    }
}

fn get_all_network_interfaces() -> Vec<(String, u64)> {
    #[cfg(not(feature = "std"))]
    {
        Vec::new()
    }

    #[cfg(feature = "std")]
    {
        let mut interfaces = Vec::new();
        if let Ok(entries) = std::fs::read_dir("/sys/class/net/") {
            for entry in entries {
                if let Ok(entry) = entry {
                    let interface_name = entry.file_name().into_string().unwrap();
                    let speed_path = format!("/sys/class/net/{}/speed", interface_name);
                    if let Ok(contents) = std::fs::read_to_string(speed_path) {
                        if let Ok(speed) = contents.trim().parse::<u64>() {
                            interfaces.push((interface_name, speed)); // Store interface and its speed in Mb/s
                        }
                    }
                }
            }
        }
        interfaces
    }
}

fn get_disk_total() -> Option<u64> {
    #[cfg(not(feature = "std"))]
    {
        None
    }

    #[cfg(feature = "std")]
    {
        let mut disk_total = DISK_TOTAL_GB.write();
        if disk_total.is_none() {
            if let Ok(contents) = read_to_string("/proc/self/mountinfo") {
                for line in contents.lines() {
                    if line.contains(" / ") {
                        // Look for the root filesystem
                        if let Ok(output) = Command::new("df").arg("/").output() {
                            let output_str = String::from_utf8_lossy(&output.stdout);
                            let lines: Vec<&str> = output_str.lines().collect();
                            if lines.len() > 1 {
                                let fields: Vec<&str> = lines[1].split_whitespace().collect();
                                if fields.len() >= 2 {
                                    *disk_total =
                                        fields[1].parse::<u64>().ok().map(|v| v / 1_000_000);
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }
        disk_total.clone()
    }
}

fn get_disk_free() -> Option<u64> {
    #[cfg(not(feature = "std"))]
    {
        None
    }

    #[cfg(feature = "std")]
    {
        if let Ok(output) = Command::new("df").arg("/").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = output_str.lines().collect();
            if lines.len() > 1 {
                let fields: Vec<&str> = lines[1].split_whitespace().collect();
                if fields.len() >= 4 {
                    return fields[3].parse::<u64>().ok().map(|v| v / 1_000_000);
                    // Convert KB to GB
                }
            }
        }
        None
    }
}
