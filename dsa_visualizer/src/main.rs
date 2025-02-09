// mod algorithm;
// mod bfs;
// use algorithm::Algorithm;
// use bfs::BFSVisualizer;
// use eframe::egui;
//
// fn main() {
//     let options = eframe::NativeOptions::default();
//     eframe::run_native(
//         "DSA Visualizer",
//         options,
//         Box::new(|_cc| Ok(Box::new(DSAVisualizer::default()))),
//     )
//         .expect("Unexpected error in running the application");
// }
//
// struct DSAVisualizer {
//     current_scene: String,
//     current_algorithm: Option<Box<dyn Algorithm>>,
// }
//
// impl Default for DSAVisualizer {
//     fn default() -> Self {
//         Self {
//             current_scene: String::new(),
//             current_algorithm: None,
//         }
//     }
// }
//
// impl eframe::App for DSAVisualizer {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             if self.current_scene.is_empty() {
//                 ui.heading("Algorithms");
//                 if ui.button("Breadth First Search (BFS)").clicked() {
//                     self.current_scene = "BFS".to_string();
//                     self.current_algorithm = Some(Box::new(BFSVisualizer::new()));
//                     self.current_algorithm.as_mut().unwrap().initialize();
//                 }
//             } else {
//                 if let Some(algorithm) = &mut self.current_algorithm {
//                     ui.heading(format!("{} Algorithm Visualization", self.current_scene));
//
//                     if ui.button("Back").clicked() {
//                         self.current_scene.clear();
//                         self.current_algorithm = None;
//                         return;
//                     }
//                     ui.separator();
//                     algorithm.render(ui);
//
//                     if ui.button("Next Step").clicked() {
//                         algorithm.step();
//                     }
//
//                     if algorithm.auto_play() {
//                         let now = std::time::Instant::now();
//                         if algorithm.last_step_time().map_or(true, |t| now.duration_since(t) >= std::time::Duration::from_secs(2)) {
//                             algorithm.step(); // Perform the next BFS step.
//                             algorithm.update_last_step_time(now); // Record the step time.
//                         }
//                     }
//                 }
//             }
//         });
//
//         if let Some(algorithm) = &mut self.current_algorithm {
//             algorithm.update();
//         }
//
//
//         ctx.request_repaint();
//     }
// }

// working example of bfs search, in the middle of large refactor
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use eframe::egui;
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "DSA Visualizer",
        options,
        Box::new(|_cc| Ok(Box::new(DSAVisualizer::default()))),
    )
        .expect("Unexpected error in running native app");
}
#[derive(Default)]
struct DSAVisualizer {
    current_scene: String,
    nodes: Vec<(usize, usize, usize)>, // Node ID, Parent ID, Level
    current_node: Option<usize>,
    visited: Vec<usize>,
    bfs_queue: VecDeque<usize>,
    tree_created: bool,
    auto_traverse: bool,
    last_step_time: Option<Instant>,
}
impl DSAVisualizer {
    fn create_example_tree(&mut self) {
        self.nodes.clear();
        self.nodes.push((1, 0, 0));
        let mut queue = VecDeque::new();
        queue.push_back((1, 0));
        let mut current_id = 2;
        while let Some((parent_id, level)) = queue.pop_front() {
            if level < 4 {
                // Create left child
                self.nodes.push((current_id, parent_id, level + 1));
                queue.push_back((current_id, level + 1));
                current_id += 1;
                // Create right child
                self.nodes.push((current_id, parent_id, level + 1));
                queue.push_back((current_id, level + 1));
                current_id += 1;
            }
        }
        self.current_node = None;
        self.visited.clear();
        self.bfs_queue.clear();
        self.tree_created = true;
        self.auto_traverse = false;
    }
    // Start BFS traversal
    fn start_bfs(&mut self) {
        if self.tree_created {
            self.visited.clear();
            self.bfs_queue.clear();
            self.current_node = None;
            self.bfs_queue.push_back(1);
            self.auto_traverse = true;
            self.last_step_time = None;
        }
    }
    fn bfs_step(&mut self) {
        if let Some(current) = self.bfs_queue.pop_front() {
            self.current_node = Some(current);
            self.visited.push(current); // Mark the node as visited
            for &(child_id, parent_id, _) in self.nodes.iter() {
                if parent_id == current && !self.visited.contains(&child_id) {
                    self.bfs_queue.push_back(child_id);
                }
            }
        } else {
            self.current_node = None;
            self.auto_traverse = false;
        }
    }
    fn render_tree(&self, ui: &mut egui::Ui) {
        // Dynamically calculate the canvas size
        let total_levels = self.nodes.iter().map(|(_, _, level)| level).max().unwrap_or(&0) + 1;
        let node_spacing = 70.0;
        let width = (2usize.pow(total_levels as u32 - 1) as f32 * node_spacing).max(800.0);
        let height = (total_levels as f32 * 120.0).max(600.0);
        let level_spacing = height / (total_levels as f32 + 1.0);
        let node_radius = 30.0;
        // Group nodes by their levels
        let mut levels: Vec<Vec<(usize, usize)>> = vec![Vec::new(); total_levels];
        for &(node_id, parent_id, level) in &self.nodes {
            levels[level].push((node_id, parent_id));
        }
        // Precompute accurate positions for nodes
        let mut positions = Vec::new();
        for (level, nodes) in levels.iter().enumerate() {
            let level_count = nodes.len();
            let x_spacing = width / (level_count as f32 + 1.0); // Spacing between nodes at this level
            for (i, &(node_id, _)) in nodes.iter().enumerate() {
                // Calculate node positions
                let x = x_spacing * (i as f32 + 1.0);
                let y = level_spacing * (level as f32 + 1.0);
                positions.push((node_id, x, y));
            }
        }
        // Draw nodes and edges
        for &(node_id, parent_id, _) in &self.nodes {
            if let Some(&(_, x, y)) = positions.iter().find(|&&(id, _, _)| id == node_id) {
                // Node color based on visitation state
                let color = if self.current_node == Some(node_id) {
                    egui::Color32::GREEN
                } else if self.visited.contains(&node_id) {
                    egui::Color32::LIGHT_BLUE
                } else {
                    egui::Color32::RED
                };
                // Draw node
                ui.painter().circle_filled(egui::pos2(x, y), node_radius, color);
                // Draw edge if it's not the root
                if parent_id != 0 {
                    if let Some(&(_, px, py)) = positions.iter().find(|&&(id, _, _)| id == parent_id) {
                        ui.painter().line_segment(
                            [egui::pos2(px, py), egui::pos2(x, y)],
                            egui::Stroke::new(2.0, egui::Color32::GRAY),
                        );
                    }
                }
                // Render node label
                ui.painter().text(
                    egui::pos2(x, y - 10.0),
                    egui::Align2::CENTER_BOTTOM,
                    format!("{}", node_id),
                    egui::TextStyle::Body.resolve(ui.style()),
                    egui::Color32::BLACK,
                );
            }
        }
    }
}
impl eframe::App for DSAVisualizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                if self.current_scene.is_empty() {
                    ui.heading("Algorithms");
                    if ui.button("Breadth First Search (BFS)").clicked() {
                        self.current_scene = "BFS".to_string();
                        self.create_example_tree();
                    }
                } else if self.current_scene == "BFS" {
                    ui.heading("Breadth First Search (BFS) Visualization");
                    if ui.button("Back").clicked() {
                        self.current_scene.clear();
                        return;
                    }
                    ui.separator();
                    // Render tree
                    ui.label("Tree Visualization:");
                    self.render_tree(ui);
                    // BFS controls
                    ui.separator();
                    if ui.button("Start BFS").clicked() {
                        self.start_bfs();
                    }
                    if ui.button("Next Step").clicked() {
                        self.bfs_step();
                    }
                }
            });
        });
        if self.auto_traverse {
            if self.last_step_time.map_or(true, |t| now.duration_since(t) >= Duration::from_secs(2)) {
                self.bfs_step();
                self.last_step_time = Some(now);
            }
        }
        ctx.request_repaint();
    }
}