use events::sdl_binding::event_manager::EventManager;
use events::sdl_binding::window_events::WindowEvent;
use rendering::shaders::shaders_utility::Shader;
use rendering::window_handler::window_handler::WindowHandler;
use std::ffi::CString;
use std::time::Duration;
use std::time::Instant;

fn main() {
    let window_handler = create_app_window();
    start_game_loop(window_handler);
}

fn create_app_window() -> WindowHandler {
    let window_name = "We are in testing grounds here my man".to_string();
    let window_with: u32 = 600;
    let window_height: u32 = 600;

    let window_handler = WindowHandler::new(window_name, window_with, window_height);
    return window_handler;
}

fn start_game_loop(mut window_handler: WindowHandler) {
    let shader_program = Shader::new(
        "/src/shaders/test_shader/vertex_2.glsl",
        "/src/shaders/test_shader/fragment_1.glsl",
    );
    let vao = prepare_3d_box();

    let mut event_manager: EventManager = EventManager::new();
    let mut game_running = true;
    let start = Instant::now();

    while game_running {
        {
            let event_pump = window_handler.event_pump();
            event_manager.match_event_types(event_pump);
        }
        game_running = !check_closed_requested(&mut event_manager);

        let elapsed = start.elapsed().as_secs_f32();
        draw_3d_box(&mut window_handler, &shader_program, vao, elapsed);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // Sleep is not accurate in timing it has a 1ms to 15 ms overshoot apparently, and 1/20th of a sec is ~50 ms
    }
}

fn check_closed_requested(event_manager: &mut EventManager) -> bool {
    let window_queue = event_manager.window_events_mut();
    while !window_queue.is_empty() {
        let event = window_queue.pop();
        match event {
            Some(WindowEvent::CloseRequested {
                timestamp: _,
                window_id: _,
            }) => return true,
            _ => {}
        }
    }
    return false;
}

type Vertex = [f32; 3];

//uses vertex 1 and fragment_1
#[allow(dead_code)]
fn draw_hello_triangle(window_handler: &mut WindowHandler, shader_program: &Shader, vao: u32) {
    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        shader_program.use_program();
        gl::BindVertexArray(vao);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);

        window_handler.window().gl_swap_window();
    }
}

#[allow(dead_code)]
fn prepare_hello_triangle() -> u32 {
    const VERTICES: [Vertex; 6] = [
        [0.5, -0.5, 0.0],  // position 0
        [1.0, 0.0, 0.0],   // couleur 0
        [-0.5, -0.5, 0.0], // position 1
        [0.0, 1.0, 0.0],   // couleur 1
        [0.0, 0.5, 0.0],   // position 2
        [0.0, 0.0, 1.0],   // couleur 2
    ];
    let mut vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        let mut vao = 0; //Vertex array object
        gl::GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);

        gl::BindVertexArray(vao);

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (2 * size_of::<Vertex>()).try_into().unwrap(),
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (2 * size_of::<Vertex>()).try_into().unwrap(),
            12 as *const _,
        );
        gl::EnableVertexAttribArray(1);
        vao
    }
}
//uses vertex 2 and fragment1
#[allow(dead_code)]
fn prepare_3d_box() -> u32 {
    const VERTICES: [Vertex; 72] = [
        // BACK face
        [-0.5, -0.5, -0.5],
        [0.30, 0.60, 0.60],
        [0.5, -0.5, -0.5],
        [0.30, 0.60, 0.60],
        [0.5, 0.5, -0.5],
        [0.30, 0.60, 0.60],
        [0.5, 0.5, -0.5],
        [0.30, 0.60, 0.60],
        [-0.5, 0.5, -0.5],
        [0.30, 0.60, 0.60],
        [-0.5, -0.5, -0.5],
        [0.30, 0.60, 0.60],
        // FRONT face
        [-0.5, -0.5, 0.5],
        [0.55, 0.35, 0.45],
        [0.5, -0.5, 0.5],
        [0.55, 0.35, 0.45],
        [0.5, 0.5, 0.5],
        [0.55, 0.35, 0.45],
        [0.5, 0.5, 0.5],
        [0.55, 0.35, 0.45],
        [-0.5, 0.5, 0.5],
        [0.55, 0.35, 0.45],
        [-0.5, -0.5, 0.5],
        [0.55, 0.35, 0.45],
        // LEFT face
        [-0.5, 0.5, 0.5],
        [0.45, 0.55, 0.35],
        [-0.5, 0.5, -0.5],
        [0.45, 0.55, 0.35],
        [-0.5, -0.5, -0.5],
        [0.45, 0.55, 0.35],
        [-0.5, -0.5, -0.5],
        [0.45, 0.55, 0.35],
        [-0.5, -0.5, 0.5],
        [0.45, 0.55, 0.35],
        [-0.5, 0.5, 0.5],
        [0.45, 0.55, 0.35],
        // RIGHT face
        [0.5, 0.5, 0.5],
        [0.75, 0.40, 0.20],
        [0.5, 0.5, -0.5],
        [0.75, 0.40, 0.20],
        [0.5, -0.5, -0.5],
        [0.75, 0.40, 0.20],
        [0.5, -0.5, -0.5],
        [0.75, 0.40, 0.20],
        [0.5, -0.5, 0.5],
        [0.75, 0.40, 0.20],
        [0.5, 0.5, 0.5],
        [0.75, 0.40, 0.20],
        // BOTTOM face
        [-0.5, -0.5, -0.5],
        [0.60, 0.25, 0.20],
        [0.5, -0.5, -0.5],
        [0.60, 0.25, 0.20],
        [0.5, -0.5, 0.5],
        [0.60, 0.25, 0.20],
        [0.5, -0.5, 0.5],
        [0.60, 0.25, 0.20],
        [-0.5, -0.5, 0.5],
        [0.60, 0.25, 0.20],
        [-0.5, -0.5, -0.5],
        [0.60, 0.25, 0.20],
        // TOP face
        [-0.5, 0.5, -0.5],
        [0.80, 0.65, 0.25],
        [0.5, 0.5, -0.5],
        [0.80, 0.65, 0.25],
        [0.5, 0.5, 0.5],
        [0.80, 0.65, 0.25],
        [0.5, 0.5, 0.5],
        [0.80, 0.65, 0.25],
        [-0.5, 0.5, 0.5],
        [0.80, 0.65, 0.25],
        [-0.5, 0.5, -0.5],
        [0.80, 0.65, 0.25],
    ];

    unsafe {
        let mut vbo: u32 = 0; //vertex buffer object
        let mut vao = 0; //Vertex array object

        gl::GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);

        gl::GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (2 * size_of::<Vertex>()).try_into().unwrap(),
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (2 * size_of::<Vertex>()).try_into().unwrap(),
            12 as *const _,
        );
        gl::EnableVertexAttribArray(1);
        vao
    }
}

#[allow(dead_code)]
fn draw_3d_box(window_handler: &mut WindowHandler, shader: &Shader, vao: u32, elapsed_time: f32) {
    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        shader.use_program();
        let shader_program = shader.program_id;

        let mut model = glm::mat4(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );

        model = glm::ext::rotate(
            &model,
            elapsed_time * glm::radians(50.0),
            glm::vec3(0.5, 1.0, 0.0),
        );

        let mut view = glm::mat4(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        view = glm::ext::translate(&view, glm::vec3(0.0, 0.0, -3.0));
        let projection = glm::ext::perspective(glm::radians(45.0), 600.0 / 600.0, 0.1, 100.0);

        let model_loc =
            gl::GetUniformLocation(shader_program, CString::new("model").unwrap().as_ptr());
        gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, &model[0][0]);
        let view_loc =
            gl::GetUniformLocation(shader_program, CString::new("view").unwrap().as_ptr());
        gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);
        let projection_loc =
            gl::GetUniformLocation(shader_program, CString::new("projection").unwrap().as_ptr());
        gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, &projection[0][0]);

        //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        gl::BindVertexArray(vao);
        gl::Enable(gl::DEPTH_TEST);
        gl::DrawArrays(gl::TRIANGLES, 0, 72);
        window_handler.window().gl_swap_window();
    }
}
