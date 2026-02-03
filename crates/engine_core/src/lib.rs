use tracing::{info, info_span};
use window::window_handler::window_handler::WindowHandler;
use events::sdl_binding::event_manager::EventManager;
use events::sdl_binding::window_events::WindowEvent;
use std::time::Duration;
use rendering::shaders::shaders_utility::Shader;


/// Entry point of the engine core.
///
/// This function initializes the window system, creates the main game window,
/// and then starts the main game loop.  
/// It blocks the current thread until the game loop exits (for example, when a
/// close event is received).
pub fn start_engine() {
    info!("Engine core has started!");
    let window_handler = create_app_window();
    start_game_loop(window_handler);
    info!("Closing engine core")
}

/// Creates the main game window using the window handler.
fn create_app_window() -> WindowHandler {
    let window_name = "We need a name for the game engine".to_string();
    let window_with: u32 = 600;
    let window_height: u32 = 600;

    let window_handler = WindowHandler::new(window_name, window_with, window_height);
    info!("Game window created");
    return window_handler
}

/// Starts the main game loop .
///
/// During each iteration of the loop, this function:
/// - Processes incoming events via the event manager
/// - Checks for a `close_requested` event to determine when to exit
///
/// The function blocks the current thread until the game loop terminates.
fn start_game_loop(mut window_handler:WindowHandler) {
    let game_loop_span: tracing::Span = info_span!(
        "Game loop",
    );
    let _enter = game_loop_span.enter();
    info!("Starting game loop");

    let shader_program = Shader::new();
    let vao = tempo_prepare_triangle();

    let mut event_manager:EventManager = EventManager::new();
    let mut game_running = true;
    while game_running {
        {
            let event_pump = window_handler.event_pump();
            event_manager.match_event_types(event_pump);
        }
        game_running = !check_closed_requested(& mut event_manager);

        tempo_draw_triangle(&mut window_handler,&shader_program,vao);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
        // Sleep is not accurate in timing it has a 1ms to 15 ms overshoot apparently, and 1/20th of a sec is ~50 ms 
    }
    info!("Closing game loop")
}

/// Checks whether a close request event has occurred in the current event queue.
///
/// The function queries the event manager to see if the user or system has requested
/// to close the application.  
/// Returns `true` if such an event was detected, `false` otherwise.
fn check_closed_requested(event_manager: & mut EventManager) -> bool {
    let window_queue = event_manager.window_events_mut();
    while !window_queue.is_empty() {
        let event = window_queue.pop();
         match event {
            Some(WindowEvent::CloseRequested { timestamp: _, window_id: _}) =>  {
                info!("Close requested");
                return true      
            }
        _ => {}
        }
    }
    return  false;
}




type Vertex = [f32; 3];
const VERTICES: [Vertex; 6] = [
    [ 0.5, -0.5, 0.0], // position 0
    [ 1.0,  0.0, 0.0], // couleur 0
    [-0.5, -0.5, 0.0], // position 1
    [ 0.0,  1.0, 0.0], // couleur 1
    [ 0.0,  0.5, 0.0], // position 2
    [ 0.0,  0.0, 1.0], // couleur 2
];

fn tempo_draw_triangle(window_handler:&mut WindowHandler, shader_program:&Shader, vao:u32) {
    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        shader_program.use_program();
        gl::BindVertexArray(vao);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);

        window_handler.window().gl_swap_window();
    }
}

fn tempo_prepare_triangle() -> u32{
    let mut vbo = 0;
    unsafe { 
        gl::GenBuffers(1,&mut vbo);
        assert_ne!(vbo,0);

        gl::BindBuffer(gl::ARRAY_BUFFER,vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        let mut vao = 0; //Vertex array object
        gl::GenVertexArrays(1,&mut vao);
        assert_ne!(vao,0);
        
        gl::BindVertexArray(vao);

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (2*size_of::<Vertex>()).try_into().unwrap(),
            0 as *const _
        );
        gl::EnableVertexAttribArray(0); 
                
        gl::VertexAttribPointer(1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (2*size_of::<Vertex>()).try_into().unwrap(),
            12 as *const _
        );
        gl::EnableVertexAttribArray(1);  
        vao
    }
}