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
        // Dynamically adjust the canvas size
        let total_levels = self.nodes.iter().map(|(_, _, level)| level).max().unwrap_or(&0) + 1;
        let max_nodes_per_level = self
            .nodes
            .iter()
            .fold(0, |max, &(_, _, level)| {
                let count = self.nodes.iter().filter(|&&(_, _, lvl)| lvl == level).count();
                max.max(count)
            });

        let node_spacing = 70.0;
        let width = (max_nodes_per_level as f32 * node_spacing).max(800.0);
        let height = (total_levels as f32 * 120.0).max(600.0);
        let level_spacing = height / (total_levels as f32 + 1.0);
        let node_radius = 30.0;

        let mut positions = Vec::new();

        for &(node_id, parent_id, level) in &self.nodes {
            let sibling_index = self
                .nodes
                .iter()
                .filter(|&&(_, pid, lvl)| pid == parent_id && lvl == level)
                .enumerate()
                .find_map(|(index, &(id, _, _))| if id == node_id { Some(index) } else { None })
                .unwrap_or(0);

            // Calculate x and y positions dynamically
            let level_nodes_count = self
                .nodes
                .iter()
                .filter(|&&(_, _, lvl)| lvl == level)
                .count();
            let x_spacing = width / (level_nodes_count as f32 + 1.0);
            let x = x_spacing * (sibling_index as f32 + 1.0);
            let y = level_spacing * (level as f32 + 1.0);

            positions.push((node_id, x, y));

            let color = if self.current_node == Some(node_id) {
                egui::Color32::GREEN
            } else if self.visited.contains(&node_id) {
                egui::Color32::LIGHT_BLUE
            } else {
                egui::Color32::RED
            };
            ui.painter().circle_filled(egui::pos2(x, y), node_radius, color);

            // Draw the edge, if it's not the root
            if parent_id != 0 {
                if let Some(&(_, px, py)) = positions.iter().find(|(pid, _, _)| *pid == parent_id) {
                    ui.painter().line_segment(
                        [egui::pos2(px, py), egui::pos2(x, y)],
                        egui::Stroke::new(2.0, egui::Color32::GRAY),
                    );
                }
            }


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