

use sdl3;
use events::sdl_binding::event_manager::EventManager;
use tracing::{info, info_span};
use tracing_subscriber::fmt::format::FmtSpan;

//Basic main event catcher for testing purposes
pub fn main() {

    init_tracing_subscriber();
    

    let app_span = info_span!(
        "Game Engine",
    );
    let _enter = app_span.enter();

    info!("application started");

    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl3 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();
    canvas.present();

    let mut event_manager:EventManager = EventManager::new();
    event_manager.start_game_loop(sdl_context,canvas);

    info!("application closed")
}

fn init_tracing_subscriber() {

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .pretty()
        .init();

    tracing_log::LogTracer::init().ok();

}