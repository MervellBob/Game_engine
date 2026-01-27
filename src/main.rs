use engine_core;
use tracing_subscriber::fmt::format::FmtSpan;


pub fn main() {
    init_tracing_subscriber();
    engine_core::start_engine();
}

/// Basic function that creates a default tracing subscribe that outputs only in the console
fn init_tracing_subscriber() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .init();

    tracing_log::LogTracer::init().ok();
}