use crate::algorithm::Algorithm;
use eframe::egui;
use egui::{Color32, Pos2, Shape, Stroke};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::time::Instant;

pub struct DijkstraVisualizer {
    graph: HashMap<usize, Vec<(usize, usize)>>,
    distances: HashMap<usize, usize>,
    previous: HashMap<usize, Option<usize>>,
    visited: Vec<usize>,
    current: Option<usize>,
    source: usize,
    target: Option<usize>,
    is_running: bool,
    last_step_time: Option<Instant>,
    auto_play: bool,
    heap: BinaryHeap<std::cmp::Reverse<(usize, usize)>>,
}

impl DijkstraVisualizer {
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
            distances: HashMap::new(),
            previous: HashMap::new(),
            visited: Vec::new(),
            current: None,
            source: 0,
            target: None,
            is_running: false,
            last_step_time: None,
            auto_play: false,
            heap: BinaryHeap::new(),
        }
    }

    pub fn initialize_graph(&mut self, graph: HashMap<usize, Vec<(usize, usize)>>, source: usize) {
        self.graph = graph;
        self.source = source;
        self.target = None;
        self.distances.clear();
        self.previous.clear();
        self.visited.clear();
        self.current = None;
        self.heap.clear();

        for &node in self.graph.keys() {
            self.distances.insert(node, usize::MAX);
            self.previous.insert(node, None);
        }
        if self.graph.contains_key(&source) {
            self.distances.insert(source, 0);
            self.heap.push(std::cmp::Reverse((0, source)));
        }
    }
}

impl Algorithm for DijkstraVisualizer {
    fn initialize(&mut self) {
        // test graph
        let mut example_graph = HashMap::new();
        example_graph.insert(0, vec![(1, 1), (3, 4)]);
        example_graph.insert(1, vec![(4, 1)]);
        example_graph.insert(2, vec![(5, 6), (0, 5), (3, 1)]);
        example_graph.insert(3, vec![(4, 2), (3, 3)]);
        example_graph.insert(4, vec![(5, 2), (2, 1)]);
        example_graph.insert(5, vec![(3, 2)]);

        self.initialize_graph(example_graph, 0);
    }
    fn step(&mut self) {
        if !self.is_running {
            println!("Algorithm finished");
            self.auto_play = false;
            return;
        }

        if let Some(std::cmp::Reverse((cost, node))) = self.heap.pop() {
            if self.visited.contains(&node) {
                return;
            }

            println!("Processing node: {:?} with cost: {}", node, cost);

            self.visited.push(node);
            self.current = Some(node);

            if let Some(target) = self.target {
                if node == target {
                    self.is_running = false;
                    println!(
                        "Target node reached: {:?}. Algorithm terminated.",
                        target
                    );
                    return;
                }
            }

            if let Some(neighbors) = self.graph.get(&node) {
                for &(neighbor, weight) in neighbors {
                    let new_cost = cost + weight;

                    println!(
                        "Inspecting edge: {} -> {} with weight: {}. New cost would be: {}",
                        node, neighbor, weight, new_cost
                    );

                    if new_cost < *self.distances.get(&neighbor).unwrap_or(&usize::MAX) {
                        self.distances.insert(neighbor, new_cost);
                        self.previous.insert(neighbor, Some(node));

                        self.heap.push(std::cmp::Reverse((new_cost, neighbor)));

                        println!(
                            "Updated distance for node {:?} to {}. Added to heap.",
                            neighbor, new_cost
                        );
                    }
                }
            }

            println!("Finished processing node: {:?}", node);
        } else {
            // If the heap is empty, the algorithm is done
            self.is_running = false;
            println!("Algorithm completed. No further steps are required.");
        }
    }

    fn render(&mut self, ui: &mut egui::Ui) {
        let painter = ui.painter();
        let node_radius = 30.0;
        let center_x = ui.min_rect().center().x;
        let center_y = ui.min_rect().center().y;
        let graph_radius = 150.0;
        let nodes_count = self.graph.len();

        let positions: Vec<(usize, Pos2)> = self
            .graph
            .keys()
            .enumerate()
            .map(|(i, &node)| {
                let angle = std::f32::consts::PI * 2.0 * (i as f32 / nodes_count as f32);
                (
                    node,
                    Pos2::new(
                        center_x + angle.cos() * graph_radius,
                        center_y + angle.sin() * graph_radius,
                    ),
                )
            })
            .collect();

        for (&node, edges) in &self.graph {
            if let Some(from_pos) = positions.iter().find(|&&(n, _)| n == node) {
                for &(neighbor, weight) in edges {
                    if let Some(to_pos) = positions.iter().find(|&&(n, _)| n == neighbor) {

                        painter.line_segment(
                            [from_pos.1, to_pos.1],
                            Stroke::new(2.0, Color32::LIGHT_BLUE), // default edge color
                        );

                        let midpoint = Pos2::new(
                            (from_pos.1.x + to_pos.1.x) / 2.0,
                            (from_pos.1.y + to_pos.1.y) / 2.0,
                        );

                        painter.text(
                            midpoint,
                            egui::Align2::CENTER_CENTER,
                            weight.to_string(),
                            egui::FontId::new(21.0, egui::FontFamily::Proportional),
                            Color32::DARK_GRAY,
                        );
                    }
                }
            }
        }

        for (node, pos) in positions.iter() {
            let color = if Some(node) == self.current.as_ref() {
                Color32::BLUE
            } else if self.visited.contains(&node) {
                Color32::GREEN
            } else {
                Color32::GRAY
            };

            painter.add(Shape::circle_filled(*pos, node_radius, color));

            painter.text(
                *pos,
                egui::Align2::CENTER_CENTER,
                node.to_string(),
                egui::FontId::new(16.0, egui::FontFamily::Proportional),
                Color32::BLACK,
            );
        }

        ui.separator();
        if self.is_running {
            ui.label(format!("Currently Visiting: {:?}", self.current));
            ui.label(format!("Visited Nodes: {:?}", self.visited));
        }
    }

    fn auto_play(&self) -> bool {
        self.auto_play
    }

    fn toggle_auto_traverse(&mut self) {
        self.auto_play = !self.auto_play;
    }

    fn start(&mut self) {
        if !self.graph.contains_key(&self.source) {
            println!("Source node {:?} is not in the graph.", self.source);
            return;
        }

        self.is_running = true;
        self.auto_play = true;

        if self.heap.is_empty() {
            self.heap.push(std::cmp::Reverse((0, self.source)));
        }
    }

    fn last_step_time(&self) -> Option<Instant> {
        self.last_step_time
    }

    fn set_last_step_time(&mut self, time: Option<Instant>) {
        self.last_step_time = time;
    }
}