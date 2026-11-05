use crate::model::{GroupInfo, ProcessInfo};
use std::collections::HashMap;
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

        let cores = self.sys.cpus().len().max(1) as f32;

        let mut list: Vec<ProcessInfo> = self
            .sys
            .processes()
            .values()
            .map(|p| ProcessInfo {
                name: p.name().to_string_lossy().to_string(),
                cpu: p.cpu_usage() / cores,
                ram_mb: p.memory() / 1024 / 1024,
            })
            .collect();

        list.sort_by(|a, b| b.ram_mb.cmp(&a.ram_mb));
        list.truncate(count);
        list
    }

    pub fn top_groups_by_ram(&mut self, count: usize) -> Vec<GroupInfo> {
        self.sys.refresh_all();

        let cores = self.sys.cpus().len().max(1) as f32;
        let mut map: HashMap<String, GroupInfo> = HashMap::new();

        for p in self.sys.processes().values() {
            let name = p.name().to_string_lossy().to_string();
            let g = map.entry(name.clone()).or_insert(GroupInfo {
                name,
                count: 0,
                cpu: 0.0,
                ram_mb: 0,
            });
            g.count += 1;
            g.cpu += p.cpu_usage() / cores;
            g.ram_mb += p.memory();
        }

        let mut list: Vec<GroupInfo> = map.into_values().collect();
        for g in &mut list {
            g.ram_mb /= 1024 * 1024;
        }
        list.sort_by(|a, b| b.ram_mb.cmp(&a.ram_mb));
        list.truncate(count);
        list
    }
}