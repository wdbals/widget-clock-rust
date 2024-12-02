use std::env;

pub mod app;
pub mod convolutions;

const WIDTH: usize = 480;
const HEIGHT: usize = 360;

/// # Widget clock with procedural background
/// Program that show the hour and have a beautiful and customizable background
///
/// ## Authors
/// - Angel Balderas
/// - Kevin Leandro
/// - Diego Rosado
fn main() {
    // tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();

    let width = args.get(1)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(WIDTH);

    let height = args.get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(HEIGHT);

    tracing::info!("starting app");
    app::run(width, height);
    tracing::info!("Closed app");
}
