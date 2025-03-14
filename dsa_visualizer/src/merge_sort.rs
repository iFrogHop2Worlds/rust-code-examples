use crate::algorithm::Algorithm;
use eframe::egui;
use std::time::Instant;
use eframe::epaint::Color32;

pub struct MergeSortVisualizer {
    data: Vec<i32>,
    temp: Vec<i32>,
    steps: Vec<(usize, usize)>,
    current_step: usize,
    current_ranges: Option<(usize, usize, usize)>, // Active ranges (start, mid, end)
    auto_play: bool,
    last_step_time: Option<Instant>,
}

impl MergeSortVisualizer {
    pub fn new() -> Self {
        let data = vec![45, 23, 11, 89, 77, 98, 4, 28, 65, 43, 56, 12, 35, 67, 99, 21, 74, 3, 18, 40, 81, 66, 29, 92, 7]; // Example dataset
        let temp = data.clone();
        let mut steps = Vec::new();
        Self::generate_steps(&mut data.clone(), &mut steps, 0, data.len() - 1);

        Self {
            data,
            temp,
            steps,
            current_step: 0,
            current_ranges: None,
            auto_play: false,
            last_step_time: None,
        }
    }

    fn generate_steps(data: &mut Vec<i32>, steps: &mut Vec<(usize, usize)>, left: usize, right: usize) {
        if left >= right {
            return;
        }
        let mid = left + (right - left) / 2;
        Self::generate_steps(data, steps, left, mid);
        Self::generate_steps(data, steps, mid + 1, right);
        steps.push((left, right)); // Save this merge step
    }

    fn merge(&mut self, left: usize, right: usize) {
        let mid = left + (right - left) / 2;
        let mut i = left;
        let mut j = mid + 1;
        let mut k = left;

        while i <= mid && j <= right {
            if self.data[i] <= self.data[j] {
                self.temp[k] = self.data[i];
                i += 1;
            } else {
                self.temp[k] = self.data[j];
                j += 1;
            }
            k += 1;
        }

        while i <= mid {
            self.temp[k] = self.data[i];
            i += 1;
            k += 1;
        }

        while j <= right {
            self.temp[k] = self.data[j];
            j += 1;
            k += 1;
        }

        for idx in left..=right {
            self.data[idx] = self.temp[idx];
        }
    }
}

impl Algorithm for MergeSortVisualizer {
    fn initialize(&mut self) {
        self.current_step = 0;
        self.auto_play = false;
        self.last_step_time = None;
    }

    fn step(&mut self) {
        if self.current_step < self.steps.len() {
            let (left, right) = self.steps[self.current_step];
            let mid = left + (right - left) / 2;
            self.merge(left, right);
            self.current_step += 1;

            // Update current ranges for rendering
            self.current_ranges = Some((left, mid + 1, right + 1)); // mid + 1 is the boundary
        } else {
            self.current_ranges = None;
        }
    }

    fn render(&mut self, ui: &mut egui::Ui) {
        let available_width = ui.available_width();
        let available_height = 200.0;
        let bar_spacing = 2.0;
        let max_value = *self.data.iter().max().unwrap_or(&1) as f32;

        let bar_width = (available_width / self.data.len() as f32) - bar_spacing;

        let (response, painter) = ui.allocate_painter(
            egui::vec2(available_width, available_height),
            egui::Sense::hover(),
        );

        let rect = response.rect;

        for (i, &value) in self.data.iter().enumerate() {
            let value_normalized = value as f32 / max_value;
            let height = value_normalized * available_height;
            let x = rect.min.x + (i as f32 * (bar_width + bar_spacing));

            let bar_rect = egui::Rect::from_min_size(
                egui::pos2(x, rect.max.y - height),
                egui::vec2(bar_width, height),
            );

            let color = if let Some((start, mid, end)) = self.current_ranges {
                if i >= start && i < mid {
                    Color32::from_rgb(0, 255, 0) // Left subarray
                } else if i >= mid && i < end {
                    Color32::from_rgb(255, 0, 0) // Right subarray
                } else {
                    Color32::from_rgb(150, 150, 150) // Inactive elements
                }
            } else {
                Color32::from_rgb(150, 150, 150)
            };

            painter.rect_filled(bar_rect, 0.0, color);
        }

        ui.separator();
        ui.label(format!(
            "Step {}/{}",
            self.current_step,
            self.steps.len()
        ));
    }

    fn auto_play(&self) -> bool {
        self.auto_play
    }

    fn toggle_auto_traverse(&mut self) {
        self.auto_play = !self.auto_play;
    }

    fn start(&mut self) {
        self.initialize();
        self.auto_play = true;
    }

    fn last_step_time(&self) -> Option<Instant> {
        self.last_step_time
    }

    fn set_last_step_time(&mut self, time: Option<Instant>) {
        self.last_step_time = time;
    }
}