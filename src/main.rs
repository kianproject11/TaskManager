mod collector;
mod model;

use collector::Collector;
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
}

impl TaskManagerApp {
    fn new() -> Self {
        Self {
            collector: Collector::new(),
        }
    }
}

impl eframe::App for TaskManagerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Task Manager");
        ui.label("Lightweight & Modern Process Monitor");
        ui.separator();

        let groups = self.collector.top_groups_by_ram(10);

        ui.add_space(5.0);

        // Modern Table Layout
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("process_grid")
                .num_columns(3)
                .spacing([40.0, 10.0])
                .striped(true)
                .show(ui, |ui| {
                    // Table Header
                    ui.strong("Process Name");
                    ui.strong("CPU Usage");
                    ui.strong("Memory (RAM)");
                    ui.end_row();

                    for g in groups {
                        ui.label(format!("{} (x{})", g.name, g.count));
                        ui.label(format!("{:.1}%", g.cpu));
                        ui.label(format!("{} MB", g.ram_mb));
                        ui.end_row();
                    }
                });
        });
    }
}