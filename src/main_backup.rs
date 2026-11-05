mod collector;
mod model;

use collector::{Collector, SortMode};
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Task Manager",
        options,
        Box::new(|_cc| {
            Ok(Box::new(TaskManagerApp::new()))
        }),
    )
}

struct TaskManagerApp {
    collector: Collector,
    search_query: String,
    sort_mode: SortMode,
    diagnosis_text: Option<String>,
}

impl TaskManagerApp {
    fn new() -> Self {
        Self {
            collector: Collector::new(),
            search_query: String::new(),
            sort_mode: SortMode::Ram,
            diagnosis_text: None,
        }
    }
}

impl eframe::App for TaskManagerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.add_space(5.0);
        ui.heading("Task Manager");
        ui.label("Lightweight & Modern Process Monitor");
        ui.separator();

        // System performance stats
        let cpu_usage = self.collector.global_cpu_usage();
        let (used_ram, total_ram) = self.collector.memory_usage();
        let ram_fraction = if total_ram > 0.0 { used_ram / total_ram } else { 0.0 };

        ui.group(|ui| {
            ui.strong("System Performance");
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.label(format!("CPU Usage: {:.1}%", cpu_usage));
                let cpu_progress = (cpu_usage / 100.0).clamp(0.0, 1.0);
                ui.add(egui::ProgressBar::new(cpu_progress).show_percentage());
            });

            ui.add_space(2.0);

            ui.horizontal(|ui| {
                ui.label(format!("RAM Usage: {:.1} GB / {:.1} GB", used_ram, total_ram));
                ui.add(egui::ProgressBar::new(ram_fraction).show_percentage());
            });
        });

        ui.add_space(8.0);

        // Action bar: Analyze Button + Search + Sort
        ui.horizontal(|ui| {
            if ui.button("🔍 Why is the system slow?").clicked() {
                self.diagnosis_text = Some(self.collector.diagnose_system());
            }

            ui.separator();

            ui.label("Search:");
            ui.text_edit_singleline(&mut self.search_query);
            if ui.button("Clear").clicked() {
                self.search_query.clear();
            }

            ui.separator();

            ui.label("Sort:");
            if ui.selectable_label(self.sort_mode == SortMode::Ram, "💾 RAM").clicked() {
                self.sort_mode = SortMode::Ram;
            }
            if ui.selectable_label(self.sort_mode == SortMode::Cpu, "⚡ CPU").clicked() {
                self.sort_mode = SortMode::Cpu;
            }
        });

        // Show Diagnosis Panel safely by handling close action outside closure borrow
        let mut close_diagnosis = false;
        if let Some(ref report) = self.diagnosis_text {
            let report_clone = report.clone(); // Clone string to avoid borrow conflict
            ui.add_space(5.0);
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.strong("💡 System Diagnosis Report");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("❌ Close Report").clicked() {
                            close_diagnosis = true;
                        }
                    });
                });
                ui.add_space(4.0);
                ui.label(report_clone);
            });
        }

        if close_diagnosis {
            self.diagnosis_text = None;
        }

        ui.add_space(5.0);
        ui.separator();

        // Fetch groups and filter
        let groups = self.collector.top_groups(50, self.sort_mode);

        let query = self.search_query.to_lowercase();
        let filtered_groups: Vec<_> = groups
            .into_iter()
            .filter(|g| query.is_empty() || g.name.to_lowercase().contains(&query))
            .collect();

        // Table Layout
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("process_grid")
                .num_columns(4)
                .spacing([40.0, 12.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.strong("Process Name");
                    ui.strong("CPU Usage");
                    ui.strong("Memory (RAM)");
                    ui.strong("Action");
                    ui.end_row();

                    let mut process_to_kill: Option<String> = None;

                    for g in filtered_groups {
                        ui.label(format!("{} (x{})", g.name, g.count));
                        ui.label(format!("{:.1}%", g.cpu));
                        ui.label(format!("{} MB", g.ram_mb));
                        
                        if ui.button("❌ End Task").clicked() {
                            process_to_kill = Some(g.name.clone());
                        }
                        ui.end_row();
                    }

                    if let Some(name) = process_to_kill {
                        self.collector.kill_process_by_name(&name);
                    }
                });
        });

        ui.ctx().request_repaint();
    }
}