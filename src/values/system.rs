use sysinfo::{Disks, System};

use crate::{
    color::colorize_range,
    status::{Markup, StatusData},
};

/// 1 GiB
const GIB: f64 = 1_073_741_824.0;

/// Returns the systems memory usage in the format `<used>/<total> GiB` in GiB as status data.
/// If the used memory exceeds 50% it will be colored yellow, if it exceeds 80% red.
pub fn memory_usage(sys: &mut System) -> StatusData {
    sys.refresh_memory();

    let used = sys.used_memory() as f64 / GIB;
    let total = sys.total_memory() as f64 / GIB;
    let ratio = used / total;

    StatusData {
        name: "memory_usage".to_string(),
        color: None,
        markup: Markup::Pango,
        full_text: colorize_range(
            format!("{:.1}/{:.1} GiB", used, total).as_str(),
            1.0 - ratio,
            (0.2, 0.5),
        ),
    }
}

/// Returns the systems available main disk space in the format `<available> GiB` in GiB as status data.
/// If the used disk space exceeds 70% it will be colored yellow, if it exceeds 90% red.
pub fn available_disk_total(disks: &mut Disks) -> Option<StatusData> {
    disks.refresh(true);
    let list = disks.list();

    if list.is_empty() {
        return None;
    }

    let available: f64 = list[0].available_space() as f64 / GIB;
    let total: f64 = list[0].total_space() as f64 / GIB;
    let available_ratio: f64 = available / total;

    Some(StatusData {
        name: "available_disk_total".to_string(),
        color: None,
        markup: Markup::Pango,
        full_text: colorize_range(
            format!("{:.1} GiB", available).as_str(),
            available_ratio,
            (0.1, 0.3),
        ),
    })
}

/// Returns the systems used main disk space in the format `<used>%` in % as status data.
/// If the used disk space exceeds 70% it will be colored yellow, if it exceeds 90% red.
pub fn disk_usage(disks: &mut Disks) -> Option<StatusData> {
    disks.refresh(true);
    let list = disks.list();

    if list.is_empty() {
        return None;
    }

    let available: f64 = list[0].available_space() as f64 / GIB;
    let total: f64 = list[0].total_space() as f64 / GIB;
    let ratio: f64 = available / total;

    Some(StatusData {
        name: "available_disk_total".to_string(),
        color: None,
        markup: Markup::Pango,
        full_text: colorize_range(
            format!("{: >4.1}%", ratio * 100.0).as_str(),
            100.0 - ratio,
            (90.0, 50.0),
        ),
    })
}

/// Returns the systems cpu usage in the format `<used>%` in % as status data.
/// If the used disk space exceeds 50% it will be colored yellow, if it exceeds 80% red.
pub fn cpu_usage(sys: &mut System) -> StatusData {
    sys.refresh_cpu_usage();
    let usage = sys.global_cpu_usage();

    StatusData {
        name: "cpu_usage".to_string(),
        color: None,
        markup: Markup::Pango,
        full_text: colorize_range(format!("{: >4.1}%", usage).as_str(), usage, (0.1, 0.3)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem() {
        let mut sys = System::new_all();
        dbg!(memory_usage(&mut sys));
    }

    #[test]
    fn test_disk() {
        let mut disks = Disks::new_with_refreshed_list();
        dbg!(available_disk_total(&mut disks));
    }

    #[test]
    fn test_cpu() {
        let mut sys = System::new_all();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        dbg!(cpu_usage(&mut sys));
    }
}
