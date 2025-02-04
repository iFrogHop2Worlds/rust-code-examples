use eframe::egui;
use rand::Rng;

pub fn create_grid(n: usize, p: f64) -> Vec<Vec<char>> {
    // Define grid size
    const GRID_HEIGHT: usize = 100; // Reduce grid height to fit better into scrollable UI
    const GRID_WIDTH: usize = 420;

    // Calculate the binomial distribution
    let mut rng = rand::thread_rng();
    let mut binomial_counts = vec![0; n + 1]; // To count occurrences of k successes (0 to n)

    // Simulate the Binomial trials
    let trials = 10000; // Number of simulations
    for _ in 0..trials {
        let mut successes = 0;
        for _ in 0..n {
            if rng.random_bool(p) {
                successes += 1;
            }
        }
        binomial_counts[successes] += 1;
    }

    // Normalize the counts to fit the grid height
    let max_count = *binomial_counts.iter().max().unwrap() as f64;
    let scaled_counts: Vec<usize> = binomial_counts
        .iter()
        .map(|&count| ((count as f64 / max_count) * (GRID_HEIGHT as f64)) as usize)
        .collect();

    // Draw the distribution on the grid
    let mut grid = vec![vec![' '; GRID_WIDTH]; GRID_HEIGHT];

    // Populate the grid
    for (k, &count) in scaled_counts.iter().enumerate() {
        // Calculate column position and height for each value of k
        let col = (k as f64 / n as f64 * (GRID_WIDTH as f64)) as usize;
        let height = count.min(GRID_HEIGHT);

        for row in (GRID_HEIGHT - height)..GRID_HEIGHT {
            grid[row][col] = if height > 50 {
                '@'
            } else if height > 25 {
                '&'
            } else {
                '$'
            };
        }
    }

    grid
}

fn main() -> eframe::Result<()> {
    // Prompt the user for inputs
    let mut input = String::new();
    println!("Enter the number of trials (n): ");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a positive integer");

    input.clear();
    println!("Enter the probability of success (p between 0 and 1): ");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let p: f64 = input.trim().parse().expect("Please enter a value between 0 and 1");

    // Create the grid
    let grid = create_grid(n, p);

    // Pass the grid to the GUI
    eframe::run_native(
        "Binomial Distribution Grid",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp { grid }))),
    )
}

struct MyApp {
    grid: Vec<Vec<char>>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| { // Make the window scrollable
                for row in &self.grid {
                    let row_string: String = row.iter().collect();
                    ui.label(egui::RichText::new(row_string).monospace());
                }
            });
        });
    }
}