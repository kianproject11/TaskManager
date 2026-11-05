use crate::model::GroupInfo;
use std::collections::HashMap;
use sysinfo::System;

#[derive(PartialEq, Clone, Copy)]
pub enum SortMode {
    Ram,
    Cpu,
}

pub struct Collector {
    sys: System,
}

impl Collector {
    pub fn new() -> Self {
        let mut sys = System::new_all().expect("Could not read system info");
        sys.refresh_all();
        Collector { sys }
    }

    pub fn global_cpu_usage(&mut self) -> f32 {
        self.sys.refresh_cpu_all();
        self.sys.global_cpu_usage()
    }

    pub fn memory_usage(&mut self) -> (f32, f32) {
        self.sys.refresh_memory();
        let total_mb = self.sys.total_memory() as f32 / (1024.0 * 1024.0 * 1024.0);
        let used_mb = self.sys.used_memory() as f32 / (1024.0 * 1024.0 * 1024.0);
        (used_mb, total_mb)
    }

    // New Diagnostic function to find system bottleneck causes
    pub fn diagnose_system(&mut self) -> String {
        self.sys.refresh_all();
        let cpu = self.sys.global_cpu_usage();
        let (used_gb, total_gb) = self.memory_usage();
        let ram_percent = if total_gb > 0.0 { (used_gb / total_gb) * 100.0 } else { 0.0 };

        let cores = self.sys.cpus().len().max(1) as f32;
        let mut top_cpu_name = String::from("None");
        let mut max_cpu = 0.0f32;
        let mut top_ram_name = String::from("None");
        let mut max_ram = 0u64;

        for p in self.sys.processes().values() {
            let name = p.name().to_string_lossy().to_string();
            let p_cpu = p.cpu_usage() / cores;
            if p_cpu > max_cpu {
                max_cpu = p_cpu;
                top_cpu_name = name.clone();
            }
            let p_ram = p.memory();
            if p_ram > max_ram {
                max_ram = p_ram;
                top_ram_name = name;
            }
        }
        let max_ram_mb = max_ram / (1024 * 1024);

        let mut report = format!("📊 System Health Analysis:\n\n");
        report.push_str(&format!("• Overall CPU Load: {:.1}%\n", cpu));
        report.push_str(&format!("• Overall RAM Load: {:.1}% ({:.1} GB / {:.1} GB)\n", ram_percent, used_gb, total_gb));
        report.push_str(&format!("• Heaviest CPU Process: {} ({:.1}% CPU)\n", top_cpu_name, max_cpu));
        report.push_str(&format!("• Heaviest RAM Process: {} ({} MB RAM)\n\n", top_ram_name, max_ram_mb));

        if cpu > 80.0 {
            report.push_str(&format!("⚠️ Diagnosis: System is running slow because CPU load is very high! The process '{}' is taking up most of your processor.", top_cpu_name));
        } else if ram_percent > 85.0 {
            report.push_str(&format!("⚠️ Diagnosis: System is running slow due to RAM shortage! The process '{}' is consuming the most memory.", top_ram_name));
        } else {
            report.push_str("✅ Diagnosis: System resources look normal. No critical performance bottleneck detected right now.");
        }
        report
    }

    pub fn top_groups(&mut self, count: usize, sort_mode: SortMode) -> Vec<GroupInfo> {
        self.sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
        
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

        match sort_mode {
            SortMode::Ram => {
                list.sort_by(|a, b| b.ram_mb.cmp(&a.ram_mb));
            }
            SortMode::Cpu => {
                list.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap_or(std::cmp::Ordering::Equal));
            }
        }

        list.truncate(count);
        list
    }

    pub fn kill_process_by_name(&mut self, target_name: &str) {
        let lower_name = target_name.to_lowercase();
        if lower_name.contains("taskmanager") || lower_name.contains("cargo") {
            return;
        }

        self.sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
        for p in self.sys.processes().values() {
            let p_name = p.name().to_string_lossy();
            if p_name == target_name {
                p.kill();
            }
        }
    }
}