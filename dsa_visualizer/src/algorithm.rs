use std::time::Instant;
use eframe::egui;

/// A common interface for all algorithms to implement
pub trait Algorithm {
    /// Called to initialize or reset the algorithm, like creating a tree/graph.
    fn initialize(&mut self);

    /// Advances the algorithm by one step (e.g., traversal step).
    fn step(&mut self);

    /// Renders the algorithm's visualization to the UI.
    fn render(&self, ui: &mut egui::Ui);

    /// Whether this algorithm is in auto-play mode (e.g., traversing automatically).
    fn auto_play(&self) -> bool;
    fn is_auto_traverse_enabled(&self) -> bool;
    fn enable_auto_traverse(&mut self);
    fn disable_auto_traverse(&mut self);
    fn last_step_time(&self) -> Option<Instant>;
    fn update_last_step_time(&mut self, time: Instant);

    /// Called to update internal timers or proceed in auto-play mode.
    fn update(&mut self);
}