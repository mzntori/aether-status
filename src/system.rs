use sysinfo::{Disks, System};

use crate::color::{colorize, Color};

/// 1 GiB
const GIB: f64 = 1_073_741_824.0;

/// Returns the systems memory usage in the format `<used>/<total> GiB` in GiB.
/// If the used memory exceeds 50% it will be colored yellow, if it exceeds 80% red.
pub fn memory_usage(sys: &mut System) -> String {
    sys.refresh_memory();

    let used = sys.used_memory() as f64 / GIB;
    let total = sys.total_memory() as f64 / GIB;
    let ratio = used / total;

    let used_col: String = colorize(format!("{:.1}/{:.1} GiB", used, total), ratio, |x| {
        if x > 0.8 {
            return Color::Red;
        } else if x > 0.5 {
            return Color::Yellow;
        }

        Color::Clear
    });

    used_col
}

/// Returns the systems available main disk space in the format `<available> GiB` in GiB.
/// If the used disk space exceeds 70% it will be colored yellow, if it exceeds 90% red.
pub fn available_disk_total(disks: &mut Disks) -> Option<String> {
    disks.refresh(true);
    let list = disks.list();

    if list.is_empty() {
        return None;
    }

    let available: f64 = list[0].available_space() as f64 / GIB;
    let total: f64 = list[0].total_space() as f64 / GIB;
    let used_ratio: f64 = 1.0 - total / available;

    let available_col: String = colorize(format!("{:.1} GiB", available), used_ratio, |x| {
        if x > 0.9 {
            return Color::Red;
        } else if x > 0.7 {
            return Color::Yellow;
        };

        Color::Clear
    });

    Some(available_col)
}

/// Returns the systems used main disk space in the format `<used>%` in %.
/// If the used disk space exceeds 70% it will be colored yellow, if it exceeds 90% red.
pub fn disk_usage(disks: &mut Disks) -> Option<String> {
    disks.refresh(true);
    let list = disks.list();

    if list.is_empty() {
        return None;
    }

    let available: f64 = list[0].available_space() as f64 / GIB;
    let total: f64 = list[0].total_space() as f64 / GIB;
    let used_ratio: f64 = 1.0 - available / total;

    let used_col: String = colorize(format!("{:.1}%", used_ratio * 100.0), used_ratio, |x| {
        if x > 0.9 {
            return Color::Red;
        } else if x > 0.7 {
            return Color::Yellow;
        };

        Color::Clear
    });

    Some(used_col)
}

/// Returns the systems cpu usage in the format `<used>%` in %.
/// If the used disk space exceeds 50% it will be colored yellow, if it exceeds 80% red.
pub fn cpu_usage(sys: &mut System) -> String {
    sys.refresh_cpu_usage();

    let usage_col: String = colorize(
        format!("{: >4.1}%", sys.global_cpu_usage()),
        sys.global_cpu_usage(),
        |x| {
            if x > 80.0 {
                return Color::Red;
            } else if x > 50.0 {
                return Color::Yellow;
            };

            Color::Clear
        },
    );

    usage_col
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem() {
        let mut sys = System::new_all();
        dbg!(memory_usage(&mut sys).to_string());
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
