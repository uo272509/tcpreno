use clap::{command, Parser};
use eframe::egui;
use tcpreno::App;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[clap(
        long,
        short,
        help = "Number of cycles to calculate",
        default_value_t = 20
    )]
    cycles: usize,

    #[clap(long, short, help = "The initial threshold", default_value_t = 8)]
    threshold: u16,

    #[clap(
        long,
        short,
        value_parser,
        value_delimiter = ',',
        help = "An array of the cycles on which a loss occurs"
    )]
    losses: Vec<usize>,

    #[clap(
        long,
        short,
        default_value = "Reno",
        help = "Algorithm to use: 'Tahoe' or 'Reno'"
    )]
    algorithm: String,
}

fn main() -> Result<(), eframe::Error> {
    let args = Cli::parse();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 400.0)),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(move |cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<App>::new(App::new(
                args.cycles,
                args.threshold,
                1,
                args.losses,
                args.algorithm.into(),
            ))
        }),
    )
}
