pub mod color;
pub mod status;
pub mod values;

use chrono::prelude::*;
use clap::Parser;
use status::Status;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use sysinfo::{Disks, System};
use values::system::{available_disk_total, cpu_usage, disk_usage, memory_usage};
use values::time::datetime_local;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Output-format: must be one of either `primary` or `non-primary`
    #[arg(short, long, default_value_t = String::from("primary"))]
    format: String,
}

/// Format
#[derive(Debug, PartialEq)]
enum Format {
    Full,
    DateAndTime,
}

/// Runs the status command in a loop.
/// Results are printed in a new line after about one second.
fn run(f: Format) {
    let mut last_check = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut disks = Disks::new_with_refreshed_list();
    let mut syst = System::new_all();
    let mut status = Status::new();
    let mut buf = String::new();
    let mut line = 0;

    println!("{{\"version\":1}}"); // initial lines
    println!("[");

    loop {
        std::thread::sleep(Duration::from_millis(20));

        let check = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if check == last_check {
            continue;
        }

        let local = Local::now();

        match f {
            Format::Full => {
                // disk
                let mut disk = available_disk_total(&mut disks).unwrap_or_default();
                disk.attach_right(disk_usage(&mut disks).unwrap_or_default(), " ");
                status.data.push(disk);

                // cpu
                status.data.push(cpu_usage(&mut syst));

                // memory
                status.data.push(memory_usage(&mut syst));

                // local
                status.data.push(datetime_local(&local));
            }
            Format::DateAndTime => {
                status.data.push(datetime_local(&local));
            }
        };

        match serde_json::to_string(&status) {
            Ok(v) => buf = v,
            Err(_) => {}
        }

        if line == 0 {
            println!("{}", buf);
        } else {
            println!(",{}", buf);
        }

        last_check = check;
        status.data.clear();
        line += 1;
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
