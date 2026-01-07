use sdl3::*;
use events::sdl::get_event;
use sdl3::render::Canvas;
use sdl3::video::Window;

pub struct Context {
    sdl: Sdl,
    video: VideoSubsystem,
    canva: Canvas<Window>,
    event_pump: EventPump,
}

impl Context {
    pub fn new() -> Self {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Test", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let mut canva = window.into_canvas();
        let event_pump = sdl_context.event_pump().unwrap();

        Context {
            sdl: sdl_context,
            video: video_subsystem,
            canva: canva,
            event_pump: event_pump,
        }
    }

    pub fn clear(&mut self) {
        self.canva.clear();
    }

    pub fn update(&mut self) {
        self.canva.present();
    }
}



impl Iterator for Context {
    type Item = events::event::Event;

    fn next(&mut self) -> Option<Self::Item> {
        get_event(self.event_pump.poll_event())
    }
}
