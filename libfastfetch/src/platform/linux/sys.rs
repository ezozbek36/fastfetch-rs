//! /sys filesystem parsers for Linux
//!
//! Provides utilities for reading DMI/SMBIOS, DRM, and other /sys interfaces

use std::io;

/// Read DMI/SMBIOS information from /sys/class/dmi/id
pub mod dmi {
    use super::*;

    const DMI_PATH: &str = "/sys/class/dmi/id";

    /// Read a DMI field
    fn read_dmi_field(field: &str) -> io::Result<String> {
        let path = format!("{DMI_PATH}/{field}");
        std::fs::read_to_string(&path).map(|s| s.trim().to_string())
    }

    /// Get system product name
    pub fn product_name() -> io::Result<String> {
        read_dmi_field("product_name")
    }

    /// Get system vendor
    pub fn sys_vendor() -> io::Result<String> {
        read_dmi_field("sys_vendor")
    }

    /// Get system version
    pub fn product_version() -> io::Result<String> {
        read_dmi_field("product_version")
    }

    /// Get board name
    pub fn board_name() -> io::Result<String> {
        read_dmi_field("board_name")
    }

    /// Get board vendor
    pub fn board_vendor() -> io::Result<String> {
        read_dmi_field("board_vendor")
    }

    /// Get BIOS version
    pub fn bios_version() -> io::Result<String> {
        read_dmi_field("bios_version")
    }

    /// Get BIOS date
    pub fn bios_date() -> io::Result<String> {
        read_dmi_field("bios_date")
    }

    /// Get chassis type
    pub fn chassis_type() -> io::Result<String> {
        read_dmi_field("chassis_type")
    }
}

/// Read thermal information
pub mod thermal {
    use super::*;

    /// Get CPU temperature from thermal zone
    pub fn cpu_temp(zone: usize) -> io::Result<f64> {
        let path = format!("/sys/class/thermal/thermal_zone{zone}/temp");
        let content = std::fs::read_to_string(&path)?;
        let millidegrees = content
            .trim()
            .parse::<i64>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(millidegrees as f64 / 1000.0)
    }

    /// Get all thermal zones
    pub fn all_zones() -> io::Result<Vec<(usize, f64)>> {
        let mut zones = Vec::new();

        for i in 0..32 {
            // Try up to 32 zones
            if let Ok(temp) = cpu_temp(i) {
                zones.push((i, temp));
            } else {
                break;
            }
        }

        Ok(zones)
    }
}

/// Read block device information
pub mod block {
    use super::*;

    /// Get block device size in bytes
    pub fn device_size(device: &str) -> io::Result<u64> {
        let path = format!("/sys/block/{device}/size");
        let content = std::fs::read_to_string(&path)?;
        let sectors = content
            .trim()
            .parse::<u64>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        // Convert 512-byte sectors to bytes
        Ok(sectors * 512)
    }

    /// List all block devices
    pub fn list_devices() -> io::Result<Vec<String>> {
        let mut devices = Vec::new();

        for entry in std::fs::read_dir("/sys/block")? {
            let entry = entry?;
            if let Some(name) = entry.file_name().to_str() {
                devices.push(name.to_string());
            }
        }

        Ok(devices)
    }
}

/// Read CPU frequency information
pub mod cpufreq {
    use super::*;

    /// Get current CPU frequency in kHz for a specific CPU
    pub fn current_freq(cpu: usize) -> io::Result<u64> {
        let path = format!("/sys/devices/system/cpu/cpu{cpu}/cpufreq/scaling_cur_freq");
        let content = std::fs::read_to_string(&path)?;
        content
            .trim()
            .parse::<u64>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// Get maximum CPU frequency in kHz for a specific CPU
    pub fn max_freq(cpu: usize) -> io::Result<u64> {
        let path = format!("/sys/devices/system/cpu/cpu{cpu}/cpufreq/scaling_max_freq");
        let content = std::fs::read_to_string(&path)?;
        content
            .trim()
            .parse::<u64>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Only run on Linux with /sys
    fn test_dmi_read() {
        let _ = dmi::product_name();
        // Just check it doesn't panic
    }

    #[test]
    #[ignore]
    fn test_block_devices() {
        let devices = block::list_devices().unwrap();
        assert!(!devices.is_empty());
    }
}
