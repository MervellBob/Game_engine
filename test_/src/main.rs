use sdl3::video::GLProfile;
use sdl3::event::Event;
use std::ffi::c_void;

use sdl3::EventPump;
use sdl3::video::Window;
use sdl3::Sdl;
use sdl3::VideoSubsystem;
use sdl3::video::GLContext;

/// Modified copy of window handler crate
pub struct WindowHandler {

    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    sdl_event_pump: EventPump,
    window: Window,
    gl_context: GLContext,
}

impl WindowHandler {
    
    pub fn new() -> Self {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();


        {
            let gl_attr = video_subsystem.gl_attr();
            gl_attr.set_context_profile(GLProfile::Core);
            gl_attr.set_context_version(3, 3);
        }

        let window = video_subsystem
            .window("OpenGL 3.3 test", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        
        gl::load_with(|s| {
        video_subsystem
            .gl_get_proc_address(s)
            .map(|f| f as *const c_void)
            .unwrap_or(std::ptr::null())
    }); 

        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            sdl_context,
            video_subsystem,
            sdl_event_pump: event_pump,
            window: window,
            gl_context: gl_context
        }
    }

    // --- GETTERS ---
    pub fn event_pump(&mut self) -> &mut EventPump {
        &mut self.sdl_event_pump
    }
    pub fn context(&self) -> &Sdl {
        &self.sdl_context
    }
    pub fn video_subsystem(&self) -> &VideoSubsystem {
        &self.video_subsystem
    }
    pub fn window(&mut self) -> &mut Window {
        &mut self.window
    }
}




type Vertex = [f32; 3];
    

type TriIndexes = [u32; 3];
    

const VERT_SHADER: &str = r#"#version 330 core
    layout (location = 0) in vec3 pos;
    void main() {
        gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
    }
    "#;

const FRAG_SHADER: &str = r#"#version 330 core
    out vec4 final_color;

    void main() {
        final_color = vec4(1.0, 0.5, 0.2, 1.0);
    }
    "#;

fn two_triangle_friends() {
    const VERTICES : [Vertex;6] = [[0.7,0.7,0.0], [0.35,-0.7,0.0], [0.0,0.7,0.0], [0.0,-0.7,0.0], [-0.7,-0.7,0.0], [-0.35,0.7,0.0]];

    let mut window_handler = WindowHandler::new();
    
    unsafe {
        let mut vbo : u32 = 0; //vertex buffer object
        gl::GenBuffers(1,&mut vbo);
        
        let mut vao = 0; //Vertex array object
        gl::GenVertexArrays(1,&mut vao);
        assert_ne!(vao,0);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, size_of_val(&VERTICES) as isize,
                VERTICES.as_ptr().cast(),gl::STATIC_DRAW);

        gl::VertexAttribPointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        size_of::<Vertex>().try_into().unwrap(),
        0 as *const _,
        );
        gl::EnableVertexAttribArray(0);  
        gl::BindVertexArray(0);
        
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
    assert_ne!(vertex_shader, 0);
    
       gl::ShaderSource(
        vertex_shader,
        1,
        &(VERT_SHADER.as_bytes().as_ptr().cast()),
        &(VERT_SHADER.len().try_into().unwrap()),
        );
    
        gl::CompileShader(vertex_shader);
    let mut success = 0;
    gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
    
    if success == 0 {
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut log_len = 0_i32;
        gl::GetShaderInfoLog(
        vertex_shader,
        1024,
        &mut log_len,
        v.as_mut_ptr().cast(),
        );
        v.set_len(log_len.try_into().unwrap());
        panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
    }

     let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
    assert_ne!(fragment_shader, 0);
    
    gl::ShaderSource(
        fragment_shader,
        1,
        &(FRAG_SHADER.as_bytes().as_ptr().cast()),
        &(FRAG_SHADER.len().try_into().unwrap()),
    );

    gl::CompileShader(fragment_shader);


    let mut success = 0;
    gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
    if success == 0 {
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut log_len = 0_i32;
        gl::GetShaderInfoLog(
        fragment_shader,
        1024,
        &mut log_len,
        v.as_mut_ptr().cast(),
        );
        v.set_len(log_len.try_into().unwrap());
        panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
    }

    let shader_program = gl::CreateProgram();
    gl::AttachShader(shader_program, vertex_shader);
    gl::AttachShader(shader_program, fragment_shader);
    gl::LinkProgram(shader_program);


    let mut success = 0;
    gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
    if success == 0 {
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut log_len = 0_i32;
        gl::GetProgramInfoLog(
        shader_program,
        1024,
        &mut log_len,
        v.as_mut_ptr().cast(),
        );
        v.set_len(log_len.try_into().unwrap());
        panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
    }

    gl:: UseProgram(shader_program);

    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader);


    'running: loop {
        for e in window_handler.event_pump().poll_iter() {
            if let Event::Quit { .. } = e {
                break 'running;
            }
        }
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);

        window_handler.window().gl_swap_window();
     }
    }
}

fn hello_triangle() {

    const VERTICES: [Vertex; 4] =
    [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.5, 0.5, 0.0],[-0.5, 0.5, 0.0]];
    const INDICES: [TriIndexes;2] = [[0,1,3], [1,2,3]];

    let mut window_handler = WindowHandler::new();
    


    unsafe {

    let mut vbo : u32 = 0; //vertex buffer object
    gl::GenBuffers(1,&mut vbo);
    
    let mut ebo :u32=0; //Element buffer object
    gl::GenBuffers(1, &mut ebo);

    
    let mut vao = 0; //Vertex array object
    gl::GenVertexArrays(1,&mut vao);
    assert_ne!(vao,0);

    gl::BindVertexArray(vao);


    gl::BindBuffer(gl::ARRAY_BUFFER,vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        size_of_val(&VERTICES) as isize,
        VERTICES.as_ptr().cast(),
        gl::STATIC_DRAW,
    );

    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER,ebo);
    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size_of_val(&INDICES) as isize, INDICES.as_ptr().cast(), gl::STATIC_DRAW);

    
    gl::VertexAttribPointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        size_of::<Vertex>().try_into().unwrap(),
        0 as *const _,
        );
    gl::EnableVertexAttribArray(0);  
    gl::BindVertexArray(0);

    let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
    assert_ne!(vertex_shader, 0);
    
       gl::ShaderSource(
        vertex_shader,
        1,
        &(VERT_SHADER.as_bytes().as_ptr().cast()),
        &(VERT_SHADER.len().try_into().unwrap()),
        );
    
        gl::CompileShader(vertex_shader);
    
    let mut success = 0;
    gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
    
    if success == 0 {
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut log_len = 0_i32;
        gl::GetShaderInfoLog(
        vertex_shader,
        1024,
        &mut log_len,
        v.as_mut_ptr().cast(),
        );
        v.set_len(log_len.try_into().unwrap());
        panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
    }

     let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
    assert_ne!(fragment_shader, 0);

    gl::ShaderSource(
        fragment_shader,
        1,
        &(FRAG_SHADER.as_bytes().as_ptr().cast()),
        &(FRAG_SHADER.len().try_into().unwrap()),
    );

    gl::CompileShader(fragment_shader);



    let mut success = 0;
    gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
    if success == 0 {
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut log_len = 0_i32;
        gl::GetShaderInfoLog(
        fragment_shader,
        1024,
        &mut log_len,
        v.as_mut_ptr().cast(),
        );
        v.set_len(log_len.try_into().unwrap());
        panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
    }

    let shader_program = gl::CreateProgram();
    gl::AttachShader(shader_program, vertex_shader);
    gl::AttachShader(shader_program, fragment_shader);
    gl::LinkProgram(shader_program);



    let mut success = 0;
    gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
    if success == 0 {
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut log_len = 0_i32;
        gl::GetProgramInfoLog(
        shader_program,
        1024,
        &mut log_len,
        v.as_mut_ptr().cast(),
        );
        v.set_len(log_len.try_into().unwrap());
        panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
    }

    gl:: UseProgram(shader_program);

    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader);


    


  

    gl::PolygonMode(gl::FRONT_AND_BACK,gl::LINE);
    'running: loop {
        for e in window_handler.event_pump().poll_iter() {
            if let Event::Quit { .. } = e {
                break 'running;
            }
        }
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES,6,gl::UNSIGNED_INT,0 as *const _);

        window_handler.window().gl_swap_window();

     }
    }
}

fn real_deal_colored_triangle() {
    const VERTICES : [Vertex;6] = [[0.5,-0.5,0.0], [1.0,0.0,0.0], [-0.5,-0.5,0.0], [0.0,1.0,0.0], [0.0,0.5,0.0], [0.0,0.0,1.0]];


    const VERT_SHADER_2: &str = r#"#version 330 core
    layout (location = 0) in vec3 pos;
    layout (location = 1) in vec3 color;

    out vec3 ourColor;
    void main() {
        gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
        ourColor = color;
    }
    "#;

    const FRAG_SHADER_2: &str = r#"#version 330 core
    out vec4 final_color;
    in vec3 ourColor;

    void main() {
        final_color = vec4(ourColor, 1.0);
    }
    "#;

    let mut window_handler = WindowHandler::new();


    unsafe {


    let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
    assert_ne!(vertex_shader, 0);
    
       gl::ShaderSource(
        vertex_shader,
        1,
        &(VERT_SHADER_2.as_bytes().as_ptr().cast()),
        &(VERT_SHADER_2.len().try_into().unwrap()),
        );
    
        gl::CompileShader(vertex_shader);
    
    let mut success = 0;
    gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
    
    if success == 0 {
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut log_len = 0_i32;
        gl::GetShaderInfoLog(
        vertex_shader,
        1024,
        &mut log_len,
        v.as_mut_ptr().cast(),
        );
        v.set_len(log_len.try_into().unwrap());
        panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
    }

     let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
    assert_ne!(fragment_shader, 0);

    gl::ShaderSource(
        fragment_shader,
        1,
        &(FRAG_SHADER_2.as_bytes().as_ptr().cast()),
        &(FRAG_SHADER_2.len().try_into().unwrap()),
    );

    gl::CompileShader(fragment_shader);



    let mut success = 0;
    gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
    if success == 0 {
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut log_len = 0_i32;
        gl::GetShaderInfoLog(
        fragment_shader,
        1024,
        &mut log_len,
        v.as_mut_ptr().cast(),
        );
        v.set_len(log_len.try_into().unwrap());
        panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
    }

    let shader_program = gl::CreateProgram();
    gl::AttachShader(shader_program, vertex_shader);
    gl::AttachShader(shader_program, fragment_shader);
    gl::LinkProgram(shader_program);






        let mut vbo : u32 = 0; //vertex buffer object
        gl::GenBuffers(1,&mut vbo);
        
        let mut vao = 0; //Vertex array object
        gl::GenVertexArrays(1,&mut vao);
        assert_ne!(vao,0);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, size_of_val(&VERTICES) as isize,
                VERTICES.as_ptr().cast(),gl::STATIC_DRAW);


        gl::VertexAttribPointer(0,3,gl::FLOAT,gl::FALSE,
                (2*size_of::<Vertex>()).try_into().unwrap(),0 as *const _,);
        gl::EnableVertexAttribArray(0); 
            
        gl::VertexAttribPointer(1,3,gl::FLOAT,gl::FALSE,
                (2*size_of::<Vertex>()).try_into().unwrap(),12 as *const _,);
        gl::EnableVertexAttribArray(1);  

        

    'running: loop {
        for e in window_handler.event_pump().poll_iter() {
            if let Event::Quit { .. } = e {
                break 'running;
            }
        }
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

        window_handler.window().gl_swap_window();
     }
                
    }
}

fn main() {
    real_deal_colored_triangle()
}
