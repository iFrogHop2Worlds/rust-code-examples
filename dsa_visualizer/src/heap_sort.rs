use crate::algorithm::Algorithm;
use eframe::egui;
use rand::Rng;
use std::time::Instant;

pub struct HeapSortVisualizer {
    data: Vec<i32>,
    heap_size: usize,
    current_index: usize,
    comparing_indices: (usize, usize),
    swapping_indices: (usize, usize),
    is_auto_traversing: bool,
    last_step: Option<Instant>,
    stage: HeapSortStage,
}

#[derive(PartialEq, Debug)]
enum HeapSortStage {
    BuildHeap,
    Sorting,
    Done,
}

impl HeapSortVisualizer {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            heap_size: 0,
            current_index: 0,
            comparing_indices: (0, 0),
            swapping_indices: (0, 0),
            is_auto_traversing: false,
            last_step: None,
            stage: HeapSortStage::BuildHeap,
        }
    }

    fn heapify(&mut self, i: usize) {
        let left = 2 * i + 1;
        let right = 2 * i + 2;
        let mut largest = i;

        if left < self.heap_size && self.data[left] > self.data[largest] {
            largest = left;
        }

        if right < self.heap_size && self.data[right] > self.data[largest] {
            largest = right;
        }

        self.comparing_indices = (i, largest);

        if largest != i {
            self.swapping_indices = (i, largest);
            self.data.swap(i, largest);
            self.heapify(largest);
        }
    }
}

impl Algorithm for HeapSortVisualizer {
    fn initialize(&mut self) {
        let mut rng = rand::thread_rng();
        self.data = (0..20).map(|_| rng.gen_range(1..100)).collect();
        self.heap_size = self.data.len();
        self.current_index = self.heap_size / 2 - 1;
        self.stage = HeapSortStage::BuildHeap;
    }

    fn step(&mut self) {
        match self.stage {
            HeapSortStage::BuildHeap => {
                if self.current_index < usize::MAX {
                    self.heapify(self.current_index);
                    if self.current_index == 0 {
                        self.stage = HeapSortStage::Sorting;
                        self.current_index = self.data.len() - 1;
                    } else {
                        self.current_index -= 1;
                    }
                }
            }
            HeapSortStage::Sorting => {
                if self.current_index > 0 {
                    self.swapping_indices = (0, self.current_index);
                    self.data.swap(0, self.current_index);
                    self.heap_size -= 1;
                    self.heapify(0);
                    self.current_index -= 1;
                } else {
                    self.stage = HeapSortStage::Done;
                }
            }
            HeapSortStage::Done => {}
        }
    }

    fn render(&mut self, ui: &mut egui::Ui) {
        ui.label("Heap Sort Visualization");
        ui.label(format!("Current Stage: {:?}", self.stage));

        let canvas_size = egui::Vec2::new(800.0, 400.0);
        let (response, painter) = ui.allocate_painter(canvas_size, egui::Sense::hover());
        let rect = response.rect;

        let bar_width = rect.width() / self.data.len() as f32;
        let scale_factor = rect.height() / 100.0;

        for (i, &value) in self.data.iter().enumerate() {
            let x = rect.left() + (i as f32 * bar_width);
            let height = value as f32 * scale_factor;
            let y = rect.bottom() - height;

            let color = if i == self.comparing_indices.0 || i == self.comparing_indices.1 {
                egui::Color32::YELLOW
            } else if i == self.swapping_indices.0 || i == self.swapping_indices.1 {
                egui::Color32::RED
            } else {
                egui::Color32::BLUE
            };

            painter.rect_filled(
                egui::Rect::from_min_max(
                    egui::pos2(x, y),
                    egui::pos2(x + bar_width - 2.0, rect.bottom()),
                ),
                0.0,
                color,
            );
        }
    }

    fn auto_play(&self) -> bool {
        self.is_auto_traversing
    }

    fn toggle_auto_traverse(&mut self) {
        self.is_auto_traversing = !self.is_auto_traversing;
    }

    fn start(&mut self) {
        self.initialize();
    }

    fn last_step_time(&self) -> Option<Instant> {
        self.last_step
    }

    fn set_last_step_time(&mut self, time: Option<Instant>) {
        self.last_step = time;
    }
}