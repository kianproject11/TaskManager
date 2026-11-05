use sysinfo::System;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut sys = System::new_all().expect("Could not read system info");

    loop {
        sleep(Duration::from_secs(2));
        sys.refresh_all();

        let mut list: Vec<_> = sys.processes().values().collect();
        list.sort_by(|a, b| b.memory().cmp(&a.memory()));

        let _ = Command::new("cmd").args(["/C", "cls"]).status();
        println!("{:<30} {:>8} {:>10}", "NAME", "CPU", "RAM");

        for p in list.iter().take(10) {
            println!(
                "{:<30} {:>7.1}% {:>7} MB",
                p.name().to_string_lossy(),
                p.cpu_usage(),
                p.memory() / 1024 / 1024
            );
        }
    }
}