use std::time::Instant;
use eframe::egui;

/// A common interface for all algorithms to implement
pub trait Algorithm {
    fn initialize(&mut self);
    fn step(&mut self);
    fn render(&self, ui: &mut egui::Ui);
    fn auto_play(&self) -> bool;
    fn toggle_auto_traverse(&mut self);
    fn start(&mut self);
    fn last_step_time(&self) -> Option<Instant> {
        self.last_step_time()
    }
    fn set_last_step_time(&mut self, time: Option<Instant>);
}