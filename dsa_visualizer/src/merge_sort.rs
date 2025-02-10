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
        let data = vec![45, 23, 11, 89, 77, 98, 4, 28, 65, 43]; // Example dataset
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

    fn render(&self, ui: &mut egui::Ui) {
        if let Some((start, mid, end)) = self.current_ranges {
            for (i, &val) in self.data.iter().enumerate() {

                if i >= start && i < mid {
                    ui.colored_label(Color32::from_rgb(0, 255, 0), format!("Index  {}:  {}", i, val));
                }

                else if i >= mid && i < end {
                    ui.colored_label(Color32::from_rgb(255, 0, 0), format!("Index  {}:  {}", i, val));
                }

                else if Some(i) == Option::from(self.current_step) {
                    ui.colored_label(Color32::from_rgb(0, 128, 128), format!("Index  {}:  {}", i, val));
                } else {
                    ui.label(format!("Index  {}:  {}", i, val));
                }
            }
        } else {
            for (i, &val) in self.data.iter().enumerate() {
                ui.label(format!("Index  {}:  {}", i, val));
            }
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