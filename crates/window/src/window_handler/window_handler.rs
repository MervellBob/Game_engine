use sdl3::EventPump;
use sdl3::render::Canvas;
use sdl3::video::Window;
use sdl3::Sdl;
use sdl3::VideoSubsystem;

/// Handles the creation and management of the main game window.
///
/// `WindowHandler` wraps SDL3 functionality, providing a convenient interface
/// to create a window, manage its canvas for rendering, and handle input events
/// via an event pump.
///
/// # Example
///
/// ```rust,no_run
/// let mut window_handler = WindowHandler::new("My Game".to_string(), 800, 600);
/// let canvas = window_handler.canvas();
/// let event_pump = window_handler.event_pump();
/// ```
pub struct WindowHandler {
    window_name: String,
    window_width: u32,
    window_height: u32,
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    canvas: Canvas<Window>,
    sdl_event_pump: EventPump,
}

impl WindowHandler {
    /// Creates a new `WindowHandler` with the specified name and dimensions.
    ///
    /// This function initializes SDL3, creates a window, prepares a canvas for
    /// rendering, and sets up an event pump to handle input and system events.
    ///
    /// # Panics
    ///
    /// This function will panic if SDL3 fails to initialize, or if window creation
    /// or event pump setup fails.
    pub fn new(name: String, width: u32, height: u32) -> Self {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(&name, width, height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas();
        canvas.present();

        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            window_name: name,
            window_width: width,
            window_height: height,
            sdl_context,
            video_subsystem,
            canvas,
            sdl_event_pump: event_pump,
        }
    }

    // --- GETTERS ---

    pub fn window_name(&self) -> &str {
        &self.window_name
    }
    pub fn window_width(&self) -> u32 {
        self.window_width
    }
    pub fn window_height(&self) -> u32 {
        self.window_height
    }
    pub fn canvas(&mut self) -> &mut Canvas<Window> {
        &mut self.canvas
    }
    pub fn event_pump(&mut self) -> &mut EventPump {
        &mut self.sdl_event_pump
    }
    pub fn context(&self) -> &Sdl {
        &self.sdl_context
    }
    pub fn video_subsystem(&self) -> &VideoSubsystem {
        &self.video_subsystem
    }
}
