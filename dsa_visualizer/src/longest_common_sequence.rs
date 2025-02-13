use crate::algorithm::Algorithm;
use eframe::egui;
use egui::Color32;

#[derive(Clone, PartialEq)]
enum CellState {
    Default,
    Current,
    LCS(usize),  
}

pub struct LCSVisualizer {
    str1: String,
    str2: String,
    dp_table: Vec<Vec<usize>>,
    cell_states: Vec<Vec<CellState>>,
    current_i: usize,
    current_j: usize,
    phase: LCSPhase,
    auto_play: bool,
    last_step_time: Option<std::time::Instant>,
    lcs_result: String,
}

#[derive(PartialEq)]
enum LCSPhase {
    Building,
    Reconstructing,
    Done,
}

impl LCSVisualizer {
    pub fn new() -> Self {
        let str1 = String::from("ABCBDAB");
        let str2 = String::from("BDCAB");
        let m = str1.len();
        let n = str2.len();
        Self {
            str1,
            str2,
            dp_table: vec![vec![0; n + 1]; m + 1],
            cell_states: vec![vec![CellState::Default; n + 1]; m + 1],
            current_i: 1,
            current_j: 1,
            phase: LCSPhase::Building,
            auto_play: false,
            last_step_time: None,
            lcs_result: String::new(),
        }
    }

    fn step_build_table(&mut self) -> bool {
        let m = self.str1.len();
        let n = self.str2.len();

        if self.current_i <= m {
            if self.str1.as_bytes()[self.current_i - 1] == self.str2.as_bytes()[self.current_j - 1] {
                self.dp_table[self.current_i][self.current_j] =
                    self.dp_table[self.current_i - 1][self.current_j - 1] + 1;
                self.cell_states[self.current_i][self.current_j] =
                    CellState::LCS(self.dp_table[self.current_i][self.current_j]);
            } else {
                self.dp_table[self.current_i][self.current_j] =
                    usize::max(self.dp_table[self.current_i - 1][self.current_j],
                               self.dp_table[self.current_i][self.current_j - 1]);
                self.cell_states[self.current_i][self.current_j] = CellState::Current;
            }

            self.current_j += 1;
            if self.current_j > n {
                self.current_j = 1;
                self.current_i += 1;
            }
            true
        } else {
            self.phase = LCSPhase::Reconstructing;
            self.current_i = self.str1.len();
            self.current_j = self.str2.len();
            false
        }
    }

    fn step_reconstruct(&mut self) -> bool {
        if self.current_i > 0 && self.current_j > 0 {
            self.cell_states[self.current_i][self.current_j] = CellState::Current;

            if self.current_i > 0 && self.current_j > 0 &&
                self.str1.as_bytes()[self.current_i - 1] == self.str2.as_bytes()[self.current_j - 1] {
                self.lcs_result.insert(0, self.str1.chars().nth(self.current_i - 1).unwrap());
                self.current_i -= 1;
                self.current_j -= 1;
            } else if self.current_i > 0 && self.current_j > 0 &&
                self.dp_table[self.current_i - 1][self.current_j] >
                    self.dp_table[self.current_i][self.current_j - 1] {
                self.current_i -= 1;
            } else {
                self.current_j -= 1;
            }
            true
        } else {
            self.phase = LCSPhase::Done;
            self.auto_play = false;
            false
        }
    }
}

impl Algorithm for LCSVisualizer {
    fn initialize(&mut self) {
        let m = self.str1.len();
        let n = self.str2.len();
        self.dp_table = vec![vec![0; n + 1]; m + 1];
        self.cell_states = vec![vec![CellState::Default; n + 1]; m + 1];
        self.current_i = 1;
        self.current_j = 1;
        self.phase = LCSPhase::Building;
        self.lcs_result.clear();
    }

    fn step(&mut self) {
        match self.phase {
            LCSPhase::Building => { self.step_build_table(); }
            LCSPhase::Reconstructing => { self.step_reconstruct(); }
            LCSPhase::Done => {}
        }
    }

    fn render(&mut self, ui: &mut egui::Ui) {
        // Input strings UI
        ui.horizontal(|ui| {
            ui.label("String 1: ");
            if ui.text_edit_singleline(&mut self.str1).changed() {
                self.initialize();
            }
        });
        ui.horizontal(|ui| {
            ui.label("String 2: ");
            if ui.text_edit_singleline(&mut self.str2).changed() {
                self.initialize();
            }
        });

        ui.separator();
        ui.label("DP Table:");

        ui.horizontal(|ui| {
            ui.label("  ");
            for c in self.str2.chars() {
                ui.label(c.to_string());
            }
        });
        
        for i in 0..=self.str1.len() {
            ui.horizontal(|ui| {
                if i == 0 {
                    ui.label(" ");
                } else {
                    ui.label(self.str1.chars().nth(i-1).unwrap().to_string());
                }

                for j in 0..=self.str2.len() {
                    let text = self.dp_table[i][j].to_string();
                    match self.cell_states[i][j] {
                        CellState::Current => {
                            ui.colored_label(Color32::DARK_BLUE, text);
                        }
                        CellState::LCS(_) => {
                            ui.colored_label(Color32::BLUE, text);
                        }
                        CellState::Default => {
                            ui.label(text);
                        }
                    }
                }
            });
        }

        ui.separator();
        ui.horizontal(|ui| {
            ui.label("LCS:");
            ui.colored_label(Color32::GREEN, format!("{}", self.lcs_result));
        });
    }

    fn auto_play(&self) -> bool { self.auto_play }
    fn toggle_auto_traverse(&mut self) { self.auto_play = !self.auto_play; }
    fn start(&mut self) { self.initialize(); }
    fn last_step_time(&self) -> Option<std::time::Instant> { self.last_step_time }
    fn set_last_step_time(&mut self, time: Option<std::time::Instant>) { self.last_step_time = time; }
}