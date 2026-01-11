//! /proc filesystem parsers for Linux

use std::collections::HashMap;
use std::io;

/// Parse /proc/meminfo
pub fn parse_meminfo() -> io::Result<HashMap<String, u64>> {
    let content = std::fs::read_to_string("/proc/meminfo")?;
    let mut info = HashMap::new();

    for line in content.lines() {
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value_str = value.trim();

            // Extract numeric value (usually in kB)
            if let Some(num_str) = value_str.split_whitespace().next() {
                if let Ok(num) = num_str.parse::<u64>() {
                    info.insert(key.to_string(), num);
                }
            }
        }
    }

    Ok(info)
}

/// Parse /proc/cpuinfo
pub fn parse_cpuinfo() -> io::Result<HashMap<String, String>> {
    let content = std::fs::read_to_string("/proc/cpuinfo")?;
    let mut info = HashMap::new();

    for line in content.lines() {
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();

            // Only store first occurrence (first CPU core info)
            info.entry(key.to_string()).or_insert_with(|| value.to_string());
        }
    }

    Ok(info)
}

/// Parse /proc/uptime
pub fn parse_uptime() -> io::Result<f64> {
    let content = std::fs::read_to_string("/proc/uptime")?;

    // Format: "uptime_seconds idle_seconds"
    if let Some(uptime_str) = content.split_whitespace().next() {
        uptime_str
            .parse::<f64>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid /proc/uptime format",
        ))
    }
}

/// Parse /proc/version
pub fn parse_version() -> io::Result<String> {
    std::fs::read_to_string("/proc/version").map(|s| s.trim().to_string())
}

/// Parse /proc/loadavg
pub fn parse_loadavg() -> io::Result<(f64, f64, f64)> {
    let content = std::fs::read_to_string("/proc/loadavg")?;
    let parts: Vec<&str> = content.split_whitespace().collect();

    if parts.len() >= 3 {
        let load1 = parts[0]
            .parse::<f64>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let load5 = parts[1]
            .parse::<f64>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let load15 = parts[2]
            .parse::<f64>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok((load1, load5, load15))
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid /proc/loadavg format",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Only run on Linux with /proc
    fn test_parse_meminfo() {
        let info = parse_meminfo().unwrap();
        assert!(info.contains_key("MemTotal"));
    }

    #[test]
    #[ignore]
    fn test_parse_cpuinfo() {
        let info = parse_cpuinfo().unwrap();
        assert!(info.contains_key("model name") || info.contains_key("cpu model"));
    }
}
