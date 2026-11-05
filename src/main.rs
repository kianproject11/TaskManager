mod collector;
mod model;

use collector::{Collector, SortMode};
use eframe::egui;

const BG: egui::Color32 = egui::Color32::from_rgb(13, 19, 32);
const CARD: egui::Color32 = egui::Color32::from_rgb(23, 33, 51);
const BLUE: egui::Color32 = egui::Color32::from_rgb(55, 130, 255);
const CYAN: egui::Color32 = egui::Color32::from_rgb(35, 205, 190);
const WHITE: egui::Color32 = egui::Color32::from_rgb(235, 242, 255);
const MUTED: egui::Color32 = egui::Color32::from_rgb(145, 160, 185);

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Task Manager")
            .with_inner_size([1100.0, 760.0])
            .with_min_inner_size([760.0, 560.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Task Manager",
        options,
        Box::new(|_cc| Ok(Box::new(TaskManagerApp::new()))),
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

    fn style(ui: &mut egui::Ui) {
        let mut visuals = egui::Visuals::dark();

        visuals.panel_fill = BG;
        visuals.window_fill = CARD;
        visuals.extreme_bg_color = egui::Color32::from_rgb(10, 15, 26);
        visuals.faint_bg_color = egui::Color32::from_rgb(30, 43, 64);
        visuals.override_text_color = Some(WHITE);

        visuals.widgets.inactive.bg_fill =
            egui::Color32::from_rgb(35, 49, 72);
        visuals.widgets.inactive.fg_stroke.color = WHITE;

        visuals.widgets.hovered.bg_fill =
            egui::Color32::from_rgb(48, 75, 112);
        visuals.widgets.hovered.fg_stroke.color = WHITE;

        visuals.widgets.active.bg_fill = BLUE;
        visuals.widgets.active.fg_stroke.color = WHITE;

        ui.ctx().set_visuals(visuals);
    }

    fn card_frame() -> egui::Frame {
        egui::Frame::new()
            .fill(CARD)
            .stroke(egui::Stroke::new(
                1.0,
                egui::Color32::from_rgb(43, 58, 82),
            ))
            .corner_radius(egui::CornerRadius::same(12))
            .inner_margin(egui::Margin::same(14))
    }
}

impl eframe::App for TaskManagerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        Self::style(ui);

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(BG)
                    .inner_margin(egui::Margin::same(20)),
            )
            .show_inside(ui, |ui| {
                // Header
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.heading(
                            egui::RichText::new("TASK MANAGER")
                                .size(27.0)
                                .strong()
                                .color(WHITE),
                        );

                        ui.label(
                            egui::RichText::new(
                                "SYSTEM PERFORMANCE MONITOR",
                            )
                            .size(11.0)
                            .color(MUTED),
                        );
                    });

                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            ui.label(
                                egui::RichText::new("● LIVE")
                                    .color(CYAN)
                                    .strong(),
                            );
                        },
                    );
                });

                ui.add_space(20.0);

                // System information
                let cpu_usage = self.collector.global_cpu_usage();
                let (used_ram, total_ram) = self.collector.memory_usage();

                let cpu_fraction = (cpu_usage / 100.0).clamp(0.0, 1.0);
                let ram_fraction = if total_ram > 0.0 {
                    (used_ram / total_ram).clamp(0.0, 1.0)
                } else {
                    0.0
                };

                ui.columns(2, |columns| {
                    // CPU Card
                    Self::card_frame().show(&mut columns[0], |ui| {
                        ui.label(
                            egui::RichText::new("PROCESSOR")
                                .size(12.0)
                                .strong()
                                .color(MUTED),
                        );

                        ui.add_space(8.0);

                        ui.label(
                            egui::RichText::new(
                                format!("{:.1}%", cpu_usage),
                            )
                            .size(32.0)
                            .strong()
                            .color(BLUE),
                        );

                        ui.add_space(8.0);

                        ui.add(
                            egui::ProgressBar::new(cpu_fraction)
                                .desired_height(12.0)
                                .fill(BLUE)
                                .show_percentage(),
                        );

                        ui.add_space(5.0);
                        ui.label(
                            egui::RichText::new("CPU UTILIZATION")
                                .size(10.0)
                                .color(MUTED),
                        );
                    });

                    // RAM Card
                    Self::card_frame().show(&mut columns[1], |ui| {
                        ui.label(
                            egui::RichText::new("MEMORY")
                                .size(12.0)
                                .strong()
                                .color(MUTED),
                        );

                        ui.add_space(8.0);

                        ui.label(
                            egui::RichText::new(
                                format!("{:.1} GB", used_ram),
                            )
                            .size(32.0)
                            .strong()
                            .color(CYAN),
                        );

                        ui.add_space(8.0);

                        ui.add(
                            egui::ProgressBar::new(ram_fraction)
                                .desired_height(12.0)
                                .fill(CYAN)
                                .show_percentage(),
                        );

                        ui.add_space(5.0);
                        ui.label(
                            egui::RichText::new(
                                format!("TOTAL MEMORY: {:.1} GB", total_ram),
                            )
                            .size(10.0)
                            .color(MUTED),
                        );
                    });
                });

                ui.add_space(18.0);

                // Tools section
                Self::card_frame().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("ANALYZE SYSTEM").clicked() {
                            self.diagnosis_text =
                                Some(self.collector.diagnose_system());
                        }

                        ui.separator();

                        ui.label(
                            egui::RichText::new("SEARCH")
                                .strong()
                                .color(MUTED),
                        );

                        ui.add(
                            egui::TextEdit::singleline(
                                &mut self.search_query,
                            )
                            .hint_text("Find a process...")
                            .desired_width(180.0),
                        );

                        if ui.button("CLEAR").clicked() {
                            self.search_query.clear();
                        }
                    });

                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("SORT BY:")
                                .color(MUTED),
                        );

                        if ui
                            .selectable_label(
                                self.sort_mode == SortMode::Ram,
                                "MEMORY",
                            )
                            .clicked()
                        {
                            self.sort_mode = SortMode::Ram;
                        }

                        if ui
                            .selectable_label(
                                self.sort_mode == SortMode::Cpu,
                                "PROCESSOR",
                            )
                            .clicked()
                        {
                            self.sort_mode = SortMode::Cpu;
                        }
                    });
                });

                // Diagnosis report
                let mut close_diagnosis = false;

                if let Some(report) = self.diagnosis_text.clone() {
                    ui.add_space(12.0);

                    Self::card_frame().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("SYSTEM DIAGNOSIS")
                                    .strong()
                                    .color(CYAN),
                            );

                            ui.with_layout(
                                egui::Layout::right_to_left(
                                    egui::Align::Center,
                                ),
                                |ui| {
                                    if ui.button("CLOSE").clicked() {
                                        close_diagnosis = true;
                                    }
                                },
                            );
                        });

                        ui.separator();
                        ui.label(report);
                    });
                }

                if close_diagnosis {
                    self.diagnosis_text = None;
                }

                ui.add_space(18.0);

                // Process list heading
                ui.horizontal(|ui| {
                    ui.heading(
                        egui::RichText::new("RUNNING PROCESSES")
                            .size(18.0)
                            .strong()
                            .color(WHITE),
                    );
                });

                ui.add_space(8.0);

                // Get and filter processes
                let groups = self.collector.top_groups(50, self.sort_mode);
                let query = self.search_query.to_lowercase();

                let filtered_groups: Vec<_> = groups
                    .into_iter()
                    .filter(|g| {
                        query.is_empty()
                            || g.name.to_lowercase().contains(&query)
                    })
                    .collect();

                ui.label(
                    egui::RichText::new(
                        format!("{} process groups found", filtered_groups.len()),
                    )
                    .size(11.0)
                    .color(MUTED),
                );

                ui.add_space(8.0);

                // Process table
                let mut process_to_kill: Option<String> = None;

                Self::card_frame().show(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(360.0)
                        .show(ui, |ui| {
                            egui::Grid::new("process_grid")
                                .num_columns(4)
                                .spacing([20.0, 13.0])
                                .striped(true)
                                .show(ui, |ui| {
                                    ui.strong("PROCESS");
                                    ui.strong("CPU");
                                    ui.strong("MEMORY");
                                    ui.strong("ACTION");
                                    ui.end_row();

                                    for g in filtered_groups.iter() {
                                        ui.label(
                                            egui::RichText::new(
                                                format!("{} (x{})", g.name, g.count),
                                            )
                                            .color(WHITE),
                                        );

                                        ui.label(
                                            egui::RichText::new(
                                                format!("{:.1}%", g.cpu),
                                            )
                                            .color(BLUE),
                                        );

                                        ui.label(
                                            egui::RichText::new(
                                                format!("{} MB", g.ram_mb),
                                            )
                                            .color(CYAN),
                                        );

                                        if ui.button("END TASK").clicked() {
                                            process_to_kill =
                                                Some(g.name.clone());
                                        }

                                        ui.end_row();
                                    }
                                });
                        });
                });

                if let Some(name) = process_to_kill {
                    self.collector.kill_process_by_name(&name);
                }

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(
                            "Task Manager  |  Rust + egui",
                        )
                        .size(10.0)
                        .color(MUTED),
                    );
                });
            });

        ui.ctx().request_repaint();
    }
}