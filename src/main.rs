pub mod app;
pub mod convolutions;

const WIDTH: usize = 128;
const HEIGHT: usize = 128;

/// # Widget clock with procedural background
/// Program that show the hour and have a beautiful and customizable background
///
/// ## Authors
/// - Angel Balderas
/// - Kevin Leandro
/// - Diego Rosado
fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("starting app");
    app::run(WIDTH, HEIGHT);
    tracing::info!("Closed app");
}
