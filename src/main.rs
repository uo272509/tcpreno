use clap::{command, Parser};
use plotly::{
    common::{DashType, Line, Title},
    layout::Axis,
    Layout, Plot, Scatter,
};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[clap(long, short, help = "Number of cycles to calculate")]
    cycles: usize,

    #[clap(long, short, help = "The initial threshold")]
    threshold: u16,

    #[clap(long, short, help = "An array of the cycles on which a loss occurs")]
    losses: Vec<usize>,

    #[clap(
        long,
        short,
        default_value = "Reno",
        help = "Algorithm to use: 'Tahoe' or 'Reno'"
    )]
    algorithm: String,
}

fn main() {
    let args = Cli::parse();

    let is_reno = args.algorithm.to_ascii_lowercase().trim().eq("reno");

    let mut window_size = 1;
    let mut threshold = args.threshold;

    let mut window_sizes: Vec<u16> = vec![window_size];
    let mut thresholds: Vec<u16> = vec![threshold];

    for cycle in 0..args.cycles {
        if args.losses.contains(&cycle) {
            threshold /= 2;

            if is_reno {
                window_size = threshold;
            } else {
                window_size = 1;
            }
        } else if window_size < threshold {
            window_size *= 2;

            if window_size > threshold {
                window_size = threshold;
            }
        } else {
            window_size += 1;
            threshold = window_size;
        }

        // agregar valores de tamaño de ventana y umbral al vector correspondiente
        window_sizes.push(window_size);
        thresholds.push(threshold);
    }

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
    println!("Cycle;Window size;Threshold;Packet losses");
    for (i, (window, threshold)) in window_sizes.iter().zip(thresholds.iter()).enumerate() {
        println!(
            "{};{};{};{}",
            i,
            window,
            threshold,
            if args.losses.contains(&i) { "Loss" } else { "" }
        );
    }
}
