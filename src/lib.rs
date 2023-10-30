mod app;

pub use app::{Algorithm, App};

pub fn algorithm(
    initial_window_size: u16,
    initial_threshold: u16,
    cycles: usize,
    losses: Vec<usize>,
    is_reno: bool,
) -> (Vec<u16>, Vec<u16>) {
    let mut window_size = initial_window_size;
    let mut threshold = initial_threshold;

    let mut window_sizes: Vec<u16> = vec![window_size];
    let mut thresholds: Vec<u16> = vec![threshold];

    for cycle in 0..cycles {
        if losses.contains(&cycle) {
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

    (window_sizes, thresholds)
}

pub fn to_csv(window_sizes: Vec<u16>, thresholds: Vec<u16>, losses: Vec<usize>) -> String {
    let mut csv = String::new();
    csv.push_str("Cycle;Window size;Threshold;Packet losses\n");
    for (i, (window, threshold)) in window_sizes.iter().zip(thresholds.iter()).enumerate() {
        csv.push_str(&format!(
            "{};{};{};{}\n",
            i,
            window,
            threshold,
            if losses.contains(&i) { "Loss" } else { "" }
        ));
    }
    csv
}
