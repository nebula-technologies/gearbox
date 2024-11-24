#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "with_serde")]
use crate::prelude::serde::derive::{Deserialize, Serialize};
use core::option::Option;
#[cfg(feature = "std")]
use std::process::Command;

#[cfg(feature = "std")]
use pnet::datalink;
#[cfg(feature = "std")]
use sys_info;

static mut CACHE_INITIALIZED: bool = false;
static mut CACHE_MANUFACTURER: Option<String> = None;
static mut CACHE_MODEL: Option<String> = None;
static mut CACHE_SERIAL_NUMBER: Option<String> = None;
static mut CACHE_BIOS_VERSION: Option<String> = None;
static mut CACHE_GPU_INFO: Option<String> = None;
static mut CACHE_CPU_VENDOR: Option<String> = None;
static mut CACHE_CPU_MODEL: Option<String> = None;
static mut CACHE_CPU_CORES: Option<u32> = None;
static mut CACHE_CPU_SPEED: Option<u64> = None;
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Device {
    #[cfg_attr(
        feature = "with_serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub location: Option<String>,

    #[cfg_attr(
        feature = "with_serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub name: Option<String>,

    pub mac: Option<String>,

    #[cfg_attr(
        feature = "with_serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub model: Option<String>,

    #[cfg_attr(
        feature = "with_serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub manufacturer: Option<String>,

    #[cfg_attr(
        feature = "with_serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub serial_number: Option<String>,

    #[cfg_attr(
        feature = "with_serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub bios_version: Option<String>,

    #[cfg_attr(
        feature = "with_serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub gpu_info: Option<String>,

    #[cfg_attr(
        feature = "with_serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub storage_devices: Option<Vec<String>>,

    #[cfg_attr(
        feature = "with_serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub usb_devices: Option<Vec<String>>,

    #[cfg_attr(
        feature = "with_serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub network_interfaces: Option<Vec<String>>,

    #[cfg_attr(
        feature = "with_serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub battery_status: Option<String>,
    pub cpu_cores: Option<u32>,
    pub cpu_speed: Option<u64>,
    pub cpu_vendor: Option<String>,
    pub cpu_model: Option<String>,
}

impl Default for Device {
    fn default() -> Self {
        // Try to collect MAC address using `pnet`
        #[cfg(feature = "std")]
        let mac = {
            let interfaces = datalink::interfaces();
            interfaces
                .iter()
                .find_map(|iface| iface.mac.map(|hw_addr| hw_addr.to_string()))
        };

        #[cfg(not(feature = "std"))]
        let mac = None;

        // Access cached information or execute once and store
        #[cfg(feature = "std")]
        let (
            manufacturer,
            model,
            serial_number,
            bios_version,
            gpu_info,
            cpu_vendor,
            cpu_model,
            cpu_cores,
            cpu_speed,
        ) = {
            // Safely initialize and cache the information if it's not already set
            unsafe {
                if !CACHE_INITIALIZED {
                    CACHE_MANUFACTURER = run_command("dmidecode", &["-s", "system-manufacturer"]);
                    CACHE_MODEL = run_command("dmidecode", &["-s", "system-product-name"]);
                    CACHE_SERIAL_NUMBER = run_command("dmidecode", &["-s", "system-serial-number"]);
                    CACHE_BIOS_VERSION = run_command("dmidecode", &["-s", "bios-version"]);
                    CACHE_GPU_INFO = run_command("lshw", &["-C", "display"]);
                    (CACHE_CPU_VENDOR, CACHE_CPU_MODEL) = if let Some((v, m)) = get_cpu_info() {
                        (Some(v), Some(m))
                    } else {
                        (None, None)
                    };
                    CACHE_INITIALIZED = true;
                    CACHE_CPU_CORES = sys_info::cpu_num().ok();
                    CACHE_CPU_SPEED = sys_info::cpu_speed().ok();
                }
                (
                    CACHE_MANUFACTURER.clone(),
                    CACHE_MODEL.clone(),
                    CACHE_SERIAL_NUMBER.clone(),
                    CACHE_BIOS_VERSION.clone(),
                    CACHE_GPU_INFO.clone(),
                    CACHE_CPU_VENDOR.clone(),
                    CACHE_CPU_MODEL.clone(),
                    CACHE_CPU_CORES.clone(),
                    CACHE_CPU_SPEED.clone(),
                )
            }
        };

        #[cfg(not(feature = "std"))]
        let (manufacturer, model, serial_number, bios_version, gpu_info, cpu_vendor, cpu_model) = {
            // In no_std environments, pre-fill with None or default values
            (
                CACHED_MANUFACTURER,
                CACHED_MODEL,
                CACHED_SERIAL_NUMBER,
                CACHED_BIOS_VERSION,
                CACHED_GPU_INFO,
                CACHE_CPU_VENDOR,
                CACHE_CPU_MODEL,
                CACHE_CPU_CORES,
                CACHE_CPU_SPEED,
            )
        };

        #[cfg(feature = "std")]
        let storage_devices = run_command("lsblk", &["-o", "NAME,SIZE,TYPE", "-d"])
            .map(|s| s.lines().map(String::from).collect());

        #[cfg(not(feature = "std"))]
        let storage_devices = None;

        #[cfg(feature = "std")]
        let usb_devices = run_command("lsusb", &[]).map(|s| s.lines().map(String::from).collect());

        #[cfg(not(feature = "std"))]
        let usb_devices = None;

        #[cfg(feature = "std")]
        let battery_status = read_battery_status();

        #[cfg(not(feature = "std"))]
        let battery_status = None;

        Self {
            location: Some("Unknown Location".to_string()),
            name: Some("Unknown Device".to_string()),
            mac,
            model: model.map(|s| s.to_string()),
            manufacturer: manufacturer.map(|s| s.to_string()),
            serial_number: serial_number.map(|s| s.to_string()),
            bios_version: bios_version.map(|s| s.to_string()),
            gpu_info: gpu_info.map(|s| s.to_string()),
            storage_devices,
            usb_devices,
            network_interfaces: Some(vec![]), // Default to empty if no interfaces found
            battery_status,
            cpu_cores,
            cpu_speed,
            cpu_vendor,
            cpu_model,
        }
    }
}

// Helper function for running commands in `std` environments
#[cfg(feature = "std")]
fn run_command(cmd: &str, args: &[&str]) -> Option<String> {
    use std::process::Command;
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

#[cfg(feature = "std")]
fn get_cpu_info() -> Option<(String, String)> {
    use std::fs;

    // Read the content of /proc/cpuinfo on Linux systems
    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        let mut vendor = None;
        let mut model = None;

        for line in cpuinfo.lines() {
            if line.starts_with("vendor_id") {
                vendor = Some(line.split(':').nth(1)?.trim().to_string());
            }
            if line.starts_with("model name") {
                model = Some(line.split(':').nth(1)?.trim().to_string());
            }
            if vendor.is_some() && model.is_some() {
                break;
            }
        }

        match (vendor, model) {
            (Some(v), Some(m)) => Some((v, m)),
            _ => None,
        }
    } else {
        None
    }
}

// Read battery status from `/sys/class/power_supply` if available
#[cfg(feature = "std")]
fn read_battery_status() -> Option<String> {
    use std::fs;
    fs::read_to_string("/sys/class/power_supply/BAT0/status")
        .ok()
        .map(|s| s.trim().to_string())
}
