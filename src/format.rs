use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub fn human_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn format_time(t: SystemTime) -> String {
    let secs = t.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    let day_secs = secs % 86400;
    let days = secs / 86400;
    let (y, mo, d) = days_to_ymd(days);
    let h = day_secs / 3600;
    let mi = (day_secs % 3600) / 60;
    format!("{:04}-{:02}-{:02} {:02}:{:02}", y, mo, d, h, mi)
}

// Howard Hinnant's civil calendar algorithm (Gregorian)
fn days_to_ymd(z: u64) -> (i64, u64, u64) {
    let z = z as i64 + 719468;
    let era = if z >= 0 { z / 146097 } else { (z - 146096) / 146097 };
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::UNIX_EPOCH;

    #[test]
    fn human_size_bytes() {
        assert_eq!(human_size(0), "0 B");
        assert_eq!(human_size(512), "512 B");
        assert_eq!(human_size(1023), "1023 B");
    }

    #[test]
    fn human_size_kilobytes() {
        assert_eq!(human_size(1024), "1.0 KB");
        assert_eq!(human_size(1536), "1.5 KB");
    }

    #[test]
    fn human_size_megabytes() {
        assert_eq!(human_size(1024 * 1024), "1.0 MB");
    }

    #[test]
    fn human_size_gigabytes() {
        assert_eq!(human_size(1024 * 1024 * 1024), "1.0 GB");
    }

    #[test]
    fn format_time_unix_epoch() {
        assert_eq!(format_time(UNIX_EPOCH), "1970-01-01 00:00");
    }

    #[test]
    fn format_time_known_date() {
        // 2024-01-01 00:00:00 UTC = 1704067200 seconds since epoch
        let t = UNIX_EPOCH + std::time::Duration::from_secs(1_704_067_200);
        assert_eq!(format_time(t), "2024-01-01 00:00");
    }
}

#[cfg(unix)]
pub fn format_mode(meta: &fs::Metadata) -> String {
    let mode = meta.permissions().mode();
    let ft = if meta.is_dir() { 'd' } else { '-' };
    let bits = |shift: u32| -> String {
        let b = (mode >> shift) & 0o7;
        format!(
            "{}{}{}",
            if b & 4 != 0 { 'r' } else { '-' },
            if b & 2 != 0 { 'w' } else { '-' },
            if b & 1 != 0 { 'x' } else { '-' },
        )
    };
    format!("{}{}{}{}", ft, bits(6), bits(3), bits(0))
}

#[cfg(not(unix))]
pub fn format_mode(meta: &fs::Metadata) -> String {
    if meta.is_dir() {
        "d---------".to_string()
    } else if meta.permissions().readonly() {
        "-r--r--r--".to_string()
    } else {
        "-rw-rw-rw-".to_string()
    }
}
