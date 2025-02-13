use crate::algorithm::Algorithm;
use eframe::egui;
use std::time::Instant;

pub struct EuclideanVisualizer {
    number1: i32,
    number2: i32,
    current_a: i32,
    current_b: i32,
    steps: Vec<(i32, i32)>,
    current_step: usize,
    auto_traverse: bool,
    last_step: Option<Instant>,
    result: Option<i32>,
}

impl EuclideanVisualizer {
    pub fn new() -> Self {
        Self {
            number1: 48,
            number2: 18,
            current_a: 48,
            current_b: 18,
            steps: Vec::new(),
            current_step: 0,
            auto_traverse: false,
            last_step: None,
            result: None,
        }
    }

    fn calculate_steps(&mut self) {
        self.steps.clear();
        let mut a = self.number1;
        let mut b = self.number2;

        while b != 0 {
            self.steps.push((a, b));
            let temp = b;
            b = a % b;
            a = temp;
        }

        self.result = Some(a);
        self.current_step = 0;
        self.current_a = self.number1;
        self.current_b = self.number2;
    }
}

impl Algorithm for EuclideanVisualizer {
    fn initialize(&mut self) {
        self.calculate_steps();
    }

    fn step(&mut self) {
        if self.current_step < self.steps.len() {
            let (a, b) = self.steps[self.current_step];
            self.current_a = a;
            self.current_b = b;
            self.current_step += 1;
        }
    }

    fn render(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("First Number: ");
            if ui.add(egui::DragValue::new(&mut self.number1).speed(1.0)).changed() {
                self.calculate_steps();
            }

            ui.label("Second Number: ");
            if ui.add(egui::DragValue::new(&mut self.number2).speed(1.0)).changed() {
                self.calculate_steps();
            }
        });

        ui.add_space(20.0);

        ui.label("Current calculation:".to_string());
        if self.current_step < self.steps.len() {
            ui.label(format!("a = {}, b = {}", self.current_a, self.current_b));
            ui.label("Next: a = b, b = a % b".to_string());
        } else if let Some(result) = self.result {
            ui.label(format!("GCD({}, {}) = {}", self.number1, self.number2, result));
        }

        let available_width = ui.available_width();
        let available_height = 200.0;
        let (response, painter) = ui.allocate_painter(
            egui::vec2(available_width, available_height),
            egui::Sense::hover(),
        );

        let rect = response.rect;
        let center = rect.center();

        let max_size = (self.number1.max(self.number2) as f32).min(100.0);
        let scale = (rect.height() / 2.0) / max_size;

        let height1 = self.current_a as f32 * scale;
        let rect1 = egui::Rect::from_min_size(
            egui::pos2(center.x - 60.0, center.y - height1),
            egui::vec2(40.0, height1),
        );
        painter.rect_filled(rect1, 0.0, egui::Color32::BLUE);

        let height2 = self.current_b as f32 * scale;
        let rect2 = egui::Rect::from_min_size(
            egui::pos2(center.x + 20.0, center.y - height2),
            egui::vec2(40.0, height2),
        );
        painter.rect_filled(rect2, 0.0, egui::Color32::GREEN);
    }

    fn auto_play(&self) -> bool {
        self.auto_traverse
    }

    fn toggle_auto_traverse(&mut self) {
        self.auto_traverse = !self.auto_traverse;
    }

    fn start(&mut self) {
        self.current_step = 0;
        self.calculate_steps();
    }

    fn last_step_time(&self) -> Option<Instant> {
        self.last_step
    }

    fn set_last_step_time(&mut self, time: Option<Instant>) {
        self.last_step = time;
    }
}