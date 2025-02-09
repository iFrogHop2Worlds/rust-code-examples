mod algorithm;
mod bfs;
mod dfs;

use std::time::Duration;
use algorithm::Algorithm;
use bfs::BFSVisualizer;
use eframe::egui;
use crate::dfs::DFSVisualizer;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "DSA Visualizer",
        options,
        Box::new(|_cc| Ok(Box::new(DSAVisualizer::default()))),
    )
        .expect("Unexpected error in running the application");
}

struct DSAVisualizer {
    current_scene: String,
    current_algorithm: Option<Box<dyn Algorithm>>,
}

impl Default for DSAVisualizer {
    fn default() -> Self {
        Self {
            current_scene: String::new(),
            current_algorithm: None,
        }
    }
}

impl eframe::App for DSAVisualizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.current_scene.is_empty() {
                ui.heading("Algorithms");
                // Menu buttons
                if ui.button("Breadth First Search (BFS)").clicked() {
                    self.current_scene = "BFS".to_string();
                    self.current_algorithm = Some(Box::new(BFSVisualizer::new()));
                    self.current_algorithm.as_mut().unwrap().initialize();
                }

                if ui.button("Depth First Search (DFS)").clicked() {
                    self.current_scene = "DFS".to_string();
                    self.current_algorithm = Some(Box::new(DFSVisualizer::new()));
                    self.current_algorithm.as_mut().unwrap().initialize();
                }
            } else {
                if let Some(algorithm) = &mut self.current_algorithm {
                    ui.heading(format!("{} Algorithm Visualization", self.current_scene));

                    if ui.button("Back").clicked() {
                        self.current_scene.clear();
                        self.current_algorithm = None;
                        return;
                    }

                    ui.separator();
                    ui.add_space(12.0);

                    algorithm.render(ui);

                    if ui.button("Start").clicked() {
                        algorithm.toggle_auto_traverse();
                        algorithm.start();
                    }

                    if ui.button("Pause").clicked() {
                        algorithm.toggle_auto_traverse();
                    }

                    if ui.button("Resume").clicked() {
                        algorithm.toggle_auto_traverse();
                    }

                    if ui.button("Next Step").clicked() {
                        algorithm.step();
                    }

                }
            }
        });

        if let Some(algorithm) = &mut self.current_algorithm {
            if algorithm.auto_play() {
                let now = std::time::Instant::now();
                if algorithm.last_step_time().map_or(true, |t| now.duration_since(t) >= Duration::from_secs(2)) {
                    println!("running algorithm");
                    algorithm.step();
                    algorithm.set_last_step_time(Some(now));
                }
            }
        }

        ctx.request_repaint();
    }
}

