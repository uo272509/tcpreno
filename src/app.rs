use eframe::egui;
use egui_plot::Plot;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Algorithm {
    Reno,
    Tahoe,
}

impl From<String> for Algorithm {
    fn from(s: String) -> Self {
        if s.to_ascii_lowercase().trim().eq("reno") {
            Algorithm::Reno
        } else if s.to_ascii_lowercase().trim().eq("tahoe") {
            Algorithm::Tahoe
        } else {
            Algorithm::Tahoe
        }
    }
}

impl Algorithm {
    pub fn is_reno(&self) -> bool {
        match self {
            Self::Reno => true,
            _ => false,
        }
    }
}

pub struct App {
    pub cycles: usize,
    pub threshold: u16,
    pub window: u16,
    pub losses: Vec<usize>,
    pub algorithm: Algorithm,
    pub plot: Option<Plot>,
}

impl App {
    pub fn new(
        cycles: usize,
        threshold: u16,
        window: u16,
        losses: Vec<usize>,
        algorithm: Algorithm,
    ) -> Self {
        Self {
            cycles,
            threshold,
            window,
            losses,
            algorithm,
            plot: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("Options").show(ctx, |ui| {
            ui.add(
                egui::Slider::from_get_set(0.0..=50.0, |v| {
                    if let Some(value) = v {
                        self.cycles = value as usize;

                        // TODO: Update the plot here with the new

                        value
                    } else {
                        self.cycles as f64
                    }
                })
                .max_decimals(0)
                .text("cycles"),
            )
            .on_hover_text_at_pointer("The number of cycles to simulate.");

            ui.add(
                egui::Slider::from_get_set(0.0..=50.0, |v| {
                    if let Some(value) = v {
                        self.threshold = value as u16;

                        // TODO: Update the plot here with the new

                        value
                    } else {
                        self.threshold as f64
                    }
                })
                .max_decimals(0)
                .text("threshold"),
            )
            .on_hover_text_at_pointer("The initial threshold.");

            ui.add(
                egui::Slider::from_get_set(0.0..=100.0, |v| {
                    if let Some(value) = v {
                        self.window = value as u16;

                        // TODO: Update the plot here with the new

                        value
                    } else {
                        self.window as f64
                    }
                })
                .max_decimals(0)
                .text("window"),
            )
            .on_hover_text_at_pointer("The initial window size.");

            ui.button("Edit losses")
                .on_hover_text_at_pointer("Edit the list of losses");

            ui.horizontal(|ui| {
                ui.label("Algorithm:");
                ui.radio_value(&mut self.algorithm, Algorithm::Reno, "Reno");
                ui.radio_value(&mut self.algorithm, Algorithm::Tahoe, "Tahoe");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
        });
    }
}
