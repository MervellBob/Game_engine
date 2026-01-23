extern crate sdl3;

mod events;
use events::*;

//Basic main event catcher for testing purposes
pub fn main() {
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

}
