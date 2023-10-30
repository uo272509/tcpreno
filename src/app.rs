use eframe::egui;
use egui_plot::{Line, LineStyle, Legend, VLine};
use crate::algorithm;

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
    pub window_size_data: Vec<[f64; 2]>,
    pub threshold_data: Vec<[f64; 2]>,
}

impl App {
    pub fn new(
        cycles: usize,
        threshold: u16,
        window: u16,
        losses: Vec<usize>,
        algo: Algorithm,
    ) -> Self {
        let (data_window, data_threshold) = algorithm(
            window,
            threshold,
            cycles,
            losses.clone(),
            algo.is_reno(),
        );

        let window_data_zipped: Vec<[f64; 2]> = (0..data_window.len()).zip(data_window.iter()).map(|(x, y)| [x as f64, *y as f64]).collect();
        let threshold_data_zipped: Vec<[f64; 2]> = (0..data_threshold.len()).zip(data_threshold.iter()).map(|(x, y)| [x as f64, *y as f64]).collect();

        Self {
            cycles,
            threshold,
            window,
            losses,
            algorithm: algo,
            window_size_data: window_data_zipped,
            threshold_data: threshold_data_zipped,
        }
    }

    fn update_data(&mut self) {
        let (data_window, data_threshold) = algorithm(
            self.window,
            self.threshold,
            self.cycles,
            self.losses.clone(),
            self.algorithm.is_reno(),
        );

        self.window_size_data = (0..data_window.len()).zip(data_window.iter()).map(|(x, y)| [x as f64, *y as f64]).collect();
        self.threshold_data = (0..data_threshold.len()).zip(data_threshold.iter()).map(|(x, y)| [x as f64, *y as f64]).collect();
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("Options").show(ctx, |ui| {
            ui.heading("Options");

            ui.add(
                egui::Slider::from_get_set(5.0..=50.0, |v| {
                    if let Some(value) = v {
                        self.cycles = value as usize;

                        self.update_data();

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
                egui::Slider::from_get_set(1.0..=50.0, |v| {
                    if let Some(value) = v {
                        self.threshold = value as u16;

                        self.update_data();

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
                egui::Slider::from_get_set(1.0..=100.0, |v| {
                    if let Some(value) = v {
                        self.window = value as u16;

                        self.update_data();

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
            egui_plot::Plot::new("example_plot")
                .height(ui.available_height()/2.0)
                .show_axes(true)
                .legend(Legend::default())
                .show(ui, |plot_ui| {
                    for loss in &self.losses {
                        plot_ui.vline(VLine::new(*loss as f64).color(egui::Color32::from_rgb(210,189,57)).style(LineStyle::dotted_dense()));
                    }

                    plot_ui.line(Line::new(self.window_size_data.clone()).name("Window size"));
                    plot_ui.line(Line::new(self.threshold_data.clone()).name("Threshold").style(LineStyle::dashed_dense()));
                });
        });
    }
}
