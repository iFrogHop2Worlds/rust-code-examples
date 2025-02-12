use crate::algorithm::Algorithm;
use eframe::egui;
use egui::{Color32, Pos2, Rect, Stroke};
use std::time::Instant;

pub struct GradientDescentVisualizer {
    weights: f64,
    learning_rate: f64,
    iteration: usize,
    max_iterations: usize,
    tolerance: f64,
    is_running: bool,
    last_step_time: Option<Instant>,
    points: Vec<Pos2>,
}

impl GradientDescentVisualizer {
    pub fn new() -> Self {
        Self {
            weights: 10.0,
            learning_rate: 0.2,
            iteration: 0,
            max_iterations: 100,
            tolerance: 0.0001,
            is_running: false,
            last_step_time: None,
            points: Vec::new(),
        }
    }

    fn calculate_gradient(&self) -> f64 {
        // Example gradient for a hypothetical function f(x) = (x - 3)^2
        // Gradient is 2 * (x - 3)
        2.0 * (self.weights - 3.0)
    }

    fn update_weights(&mut self) {
        let gradient = self.calculate_gradient();
        self.weights -= self.learning_rate * gradient;
        self.iteration += 1;

        self.points.push(Pos2::new(self.weights as f32, gradient as f32));
    }

    fn is_converged(&self) -> bool {
        let gradient = self.calculate_gradient();
        gradient.abs() < self.tolerance || self.iteration >= self.max_iterations
    }

}

impl Algorithm for GradientDescentVisualizer {
    fn initialize(&mut self) {
        self.weights = 10.0;
        self.iteration = 0;
        self.is_running = false;
        self.points.clear();
        self.last_step_time = None;
    }

    fn step(&mut self) {
        if !self.is_converged() {
            self.update_weights();
        } else {
            self.is_running = false;
        }
    }

    fn render(&mut self, ui: &mut egui::Ui) {
        ui.label(format!("Weights: {:.4}", self.weights));
        ui.label(format!("Iteration: {}", self.iteration));
        ui.label(format!("Gradient: {:.4}", self.calculate_gradient()));

        if self.is_converged() {
            ui.label("Converged!");
        }

        let available_rect = ui.available_rect_before_wrap();
        let width = available_rect.width();
        let height = available_rect.height();
        let plot_left = available_rect.left();
        let plot_right = available_rect.right();
        let plot_bottom = available_rect.bottom();
        let plot_top = available_rect.top();

        // ** min and max y-values for scaling **
        let min_value = self
            .points
            .iter()
            .map(|point| point.y) // Use y-component of Pos2
            .fold(f32::INFINITY, f32::min); // Find the minimum y-value

        let max_value = self
            .points
            .iter()
            .map(|point| point.y) // Use y-component of Pos2
            .fold(f32::NEG_INFINITY, f32::max); // Find the maximum y-value

        let y_scale = |value: f32| {
            let normalized_value = (value - min_value) / (max_value - min_value + 1e-6); // Avoid division by zero
            plot_bottom - (normalized_value * height) // Map to screen-space y
        };

        let painter = ui.painter();
        painter.rect_stroke(
            Rect::from_min_max(
                Pos2::new(plot_left, plot_top),
                Pos2::new(plot_right, plot_bottom),
            ),
            0.0, // No corner rounding
            Stroke::new(2.0, Color32::BLACK),
        );


        let mut last_pixel_point: Option<Pos2> = None;

        let x_step = width / self.points.len() as f32;

        for (i, point) in self.points.iter().enumerate() {
            let x = plot_left + x_step * i as f32;
            let y = y_scale(point.y);

            let this_pixel_point = Pos2::new(x, y);

            let color = if i == self.points.len() - 1 && self.is_converged() {
                Color32::GREEN
            } else {
                Color32::RED
            };

            painter.circle_filled(this_pixel_point, 4.0, color);

            if let Some(last_pixel_point) = last_pixel_point {
                painter.line_segment(
                    [last_pixel_point, this_pixel_point],
                    Stroke::new(1.0, Color32::BLACK),
                );
            }

            last_pixel_point = Some(this_pixel_point);
        }
    }

    fn auto_play(&self) -> bool {
        self.is_running
    }

    fn toggle_auto_traverse(&mut self) {
        self.is_running = !self.is_running;
    }

    fn start(&mut self) {
        self.is_running = true;
    }

    fn last_step_time(&self) -> Option<Instant> {
        self.last_step_time
    }

    fn set_last_step_time(&mut self, time: Option<Instant>) {
        self.last_step_time = time;
    }
}