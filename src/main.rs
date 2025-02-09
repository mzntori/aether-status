mod color;
mod system;
mod time;

use chrono::prelude::*;
use clap::Parser;
use std::time::{Duration, Instant};
use sysinfo::{Disks, System};
use system::{available_disk_total, cpu_usage, disk_usage, memory_usage};
use time::{date_local, time_local};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Output-format: must be one of either `primary` or `non-primary`
    #[arg(short, long, default_value_t = String::from("primary"))]
    format: String,
}

/// Format variants
#[derive(Debug, PartialEq)]
enum Format {
    Full,
    DateAndTime,
}

/// Runs the status command in a loop.
/// Results are printed in a new line after about one second.
fn run(f: Format) {
    let mut last_check: Instant = Instant::now()
        .checked_sub(Duration::from_secs(2))
        .unwrap_or(Instant::now());
    let mut disks = Disks::new_with_refreshed_list();
    let mut syst = System::new_all();

    loop {
        std::thread::sleep(Duration::from_millis(10));

        if last_check.elapsed().as_micros() < 1_000_000 {
            continue;
        }

        let local = Local::now();

        let buf = match f {
            Format::Full => {
                format!(
                    "\n{} {} | {} | {} | {} {}",
                    available_disk_total(&mut disks).unwrap_or("N/A".to_string()),
                    disk_usage(&mut disks).unwrap_or("N/A".to_string()),
                    cpu_usage(&mut syst),
                    memory_usage(&mut syst),
                    date_local(&local),
                    time_local(&local)
                )
            }
            Format::DateAndTime => {
                format!("\n{} {}", date_local(&local), time_local(&local))
            }
        };

        print!("{}", buf);
        last_check = Instant::now();
    }
}

fn main() {
    let args = Args::parse();

    let format = match args.format.as_str() {
        "primary" => Format::Full,
        "non-primary" => Format::DateAndTime,
        _ => {
            println!("error: invalid usage: use the '--help' option to get more information");
            return;
        }
    };

    run(format);
}
