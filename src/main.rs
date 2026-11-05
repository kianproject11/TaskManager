mod collector;
mod model;

use collector::Collector;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut collector = Collector::new();

    loop {
        sleep(Duration::from_secs(2));
        let list = collector.top_groups_by_ram(10);

        let _ = Command::new("cmd").args(["/C", "cls"]).status();
        println!("{:<30} {:>5} {:>8} {:>10}", "NAME", "COUNT", "CPU", "RAM");

        for g in &list {
            println!(
                "{:<30} {:>5} {:>7.1}% {:>7} MB",
                g.name, g.count, g.cpu, g.ram_mb
            );
        }
    }
}