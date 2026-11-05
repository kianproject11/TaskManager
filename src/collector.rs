use crate::model::ProcessInfo;
use sysinfo::System;

pub struct Collector {
    sys: System,
}

impl Collector {
    pub fn new() -> Self {
        let sys = System::new_all().expect("Could not read system info");
        Collector { sys }
    }

    pub fn top_by_ram(&mut self, count: usize) -> Vec<ProcessInfo> {
        self.sys.refresh_all();

        let mut list: Vec<ProcessInfo> = self
            .sys
            .processes()
            .values()
            .map(|p| ProcessInfo {
                name: p.name().to_string_lossy().to_string(),
                cpu: p.cpu_usage(),
                ram_mb: p.memory() / 1024 / 1024,
            })
            .collect();

        list.sort_by(|a, b| b.ram_mb.cmp(&a.ram_mb));
        list.truncate(count);
        list
    }
}