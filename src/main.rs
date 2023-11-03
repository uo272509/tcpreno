// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::{command, Parser};
use tcpreno::App;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[clap(long, help = "Number of cycles to calculate", default_value_t = 20)]
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
    losses: Vec<u16>,

    #[clap(
        long,
        short,
        default_value = "Reno",
        help = "Algorithm to use: 'Tahoe' or 'Reno'"
    )]
    algorithm: String,

    #[clap(
        long,
        short,
        default_value_t = false,
        help = "Avoid the GUI: output the results to stdout and as an image"
    )]
    cli: bool,
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    let args = Cli::parse();

    if args.cli {
        do_cli(args);
        Ok(())
    } else {
        do_gui(args)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn do_gui(args: Cli) -> Result<(), eframe::Error> {
    use eframe::egui;

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1500.0, 700.0)),
        ..Default::default()
    };

    eframe::run_native(
        "TCP Reno/Tahoe",
        options,
        Box::new(move |cc| {
            //Increase the font size
            let mut st = (*egui::Context::default().style()).clone();
            st.override_font_id = Some(egui::FontId::monospace(14.0));
            st.spacing.slider_width = 250.0;
            st.spacing.button_padding = egui::Vec2::new(10.0, 5.0);
            st.spacing.item_spacing = egui::Vec2::new(10.0, 10.0);
            cc.egui_ctx.set_style(st);

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

#[cfg(not(target_arch = "wasm32"))]
fn do_cli(args: Cli) {
    use plotly::{
        common::{DashType, Line, Title},
        layout::Axis,
        Layout, Plot, Scatter,
    };
    use tcpreno::{algorithm, to_csv};

    let is_reno = args.algorithm.to_ascii_lowercase().trim().eq("reno");

    let window_size = 1;
    let threshold = args.threshold;

    let (window_sizes, thresholds): (Vec<u16>, Vec<u16>) =
        algorithm(window_size, threshold, args.cycles, &args.losses, is_reno);

    // Generar scatter plots de cada uno de los vectores
    let window_size_trace =
        Scatter::new((0..window_sizes.len()).collect(), window_sizes.clone()).name("Window size");
    let threshold_trace = Scatter::new((0..thresholds.len()).collect(), thresholds.clone())
        .line(Line::new().dash(DashType::DashDot))
        .name("Threshold");

    // Crear un plot
    let layout = Layout::new()
        .title(Title::new(&format!(
            "{} Window Size and Threshold",
            args.algorithm
        )))
        .x_axis(Axis::new().title(Title::new("Cycle")))
        .y_axis(Axis::new().title(Title::new("Window Size / Threshold")));

    let mut plot = Plot::new();
    plot.add_trace(window_size_trace);
    plot.add_trace(threshold_trace);
    plot.set_layout(layout);

    // Y mostrarlo
    plot.show();

    // Y además generar el csv
    let ws: Vec<[f64; 2]> = window_sizes
        .iter()
        .map(|v| [0.0f64, *v as f64])
        .collect::<Vec<[f64; 2]>>();

    let ts: Vec<[f64; 2]> = thresholds
        .iter()
        .map(|v| [0.0f64, *v as f64])
        .collect::<Vec<[f64; 2]>>();

    println!("{}", to_csv(&ws, &ts, &args.losses));
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::egui;

    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "Canvas", // hardcode it
                web_options,
                Box::new(move |cc| {
                    //Increase the font size
                    let mut st = (*egui::Context::default().style()).clone();
                    st.override_font_id = Some(egui::FontId::monospace(14.0));
                    st.spacing.slider_width = 250.0;
                    st.spacing.button_padding = egui::Vec2::new(10.0, 5.0);
                    st.spacing.item_spacing = egui::Vec2::new(10.0, 10.0);
                    cc.egui_ctx.set_style(st);

                    // This gives us image support:
                    egui_extras::install_image_loaders(&cc.egui_ctx);
                    Box::<App>::default()
                }),
            )
            .await
            .expect("failed to start eframe");
    });
}
