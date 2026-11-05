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
        let list = collector.top_by_ram(10);

        let _ = Command::new("cmd").args(["/C", "cls"]).status();
        println!("{:<30} {:>8} {:>10}", "NAME", "CPU", "RAM");

        for p in &list {
            println!("{:<30} {:>7.1}% {:>7} MB", p.name, p.cpu, p.ram_mb);
        }
    }
}