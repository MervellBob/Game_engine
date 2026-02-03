use std::fmt::format;
use std::fs::File;
use asset::asset::Asset;
use asset::text::Text;
use std::collections::HashMap;
use std::ffi::CString;

pub struct Shader {
    pub program_id:u32, 
    uniforms: HashMap<String, i32>,

}

impl Shader {
    pub fn new() -> Self {
        let vertex_path = format!("{}/src/shaders/test_shader/vertex_1.glsl", env!("CARGO_MANIFEST_DIR"));
        let fragment_path: String = format!("{}/src/shaders/test_shader/fragment_1.glsl", env!("CARGO_MANIFEST_DIR"));


        let vertex_shader = Self::compile_shader(&vertex_path,gl::VERTEX_SHADER);
        let fragment_shader = Self::compile_shader(&fragment_path,gl::FRAGMENT_SHADER);

        let shader_program = Self::create_program(vertex_shader,fragment_shader);

        Self { 
            program_id:shader_program,
            uniforms: HashMap::new()
        }
    }

    fn compile_shader(shader_path: &str, shader_type: u32) -> u32 {
        let error_string = format(format_args!("Unable to open shader file: {}", shader_path));
        let file = File::open(shader_path).expect(&error_string);
        let mut file: Text = Text::new(file);
        file.read_raw();
        let shader_content = String::from_utf8(file.contents_raw).expect("Found invalid UTF-8");


        unsafe {
            let shader = gl::CreateShader(shader_type);
            assert_ne!(shader, 0, "Failed to create shader");

            gl::ShaderSource(
                shader,
                1,
                &(shader_content.as_bytes().as_ptr().cast()),
                &(shader_content.len().try_into().unwrap()),
            );

            gl::CompileShader(shader);
            Self::check_compile_error(shader, "NOT PROGRAM");
            shader
        }
    }

    fn create_program(vertex_shader:u32,fragment_shader:u32) -> u32 {
        unsafe {
            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);
            Self::check_compile_error(shader_program, "PROGRAM");
            gl:: UseProgram(shader_program);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            shader_program
        }
    }
    
    // utility function for checking shader compilation/linking errors.
    // ------------------------------------------------------------------------
    fn check_compile_error(shader:u32, shader_type:&str) {
        let mut success = 0;
        let mut log_len = 0;
        match shader_type {
            "PROGRAM" => {
                unsafe {
                    gl::GetProgramiv(shader,gl::LINK_STATUS, &mut success);
                    if success == 0 {
                        gl::GetProgramiv(shader,gl::INFO_LOG_LENGTH, &mut log_len);
                        let mut log = Vec::with_capacity(log_len as usize);
                        log.resize(log_len as usize, 0);

                        gl::GetProgramInfoLog(
                        shader,
                        log_len,
                        std::ptr::null_mut(),
                        log.as_mut_ptr().cast(),
                        );
                        let log = String::from_utf8_lossy(&log);
                        panic!("Program Link Error:\n{}" , log);
                    }
                }
            }
            _ => {
                unsafe {
                    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
                    if success == 0 {
                        
                        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_len);
                        let mut log = Vec::with_capacity(log_len as usize);
                        log.resize(log_len as usize, 0);

                        gl::GetShaderInfoLog(
                            shader,
                            log_len,
                            std::ptr::null_mut(),
                            log.as_mut_ptr().cast(),
                        );

                        let log = String::from_utf8_lossy(&log);
                        panic!("Shader Compile Error:\n{}" , log);
                    }
                }
            }
            
        }

    }


    fn get_uniform_location(&mut self, uniform_name :&str) -> i32 {
        if let Some(&loc) = self.uniforms.get(uniform_name) {
            return loc;
        }
        let c_uniform_name = CString::new(uniform_name).unwrap();

        let location = unsafe {
            gl::GetUniformLocation(self.program_id, c_uniform_name.as_ptr())
        };

        self.uniforms.insert(uniform_name.to_string(), location);
        location
    }
    // activate the shader
    // ------------------------------------------------------------------------
    pub fn use_program(&self) 
    { 
        unsafe {
            gl::UseProgram(self.program_id); 
        }
    }
    // utility uniform functions
    // ------------------------------------------------------------------------
    pub fn set_bool(&mut self,uniform_name:&str, value:bool)
    {         
        let loc = self.get_uniform_location(uniform_name);
        unsafe {
            gl::Uniform1i(loc, value as i32);
        }
    }
    // ------------------------------------------------------------------------
    pub fn set_int(&mut self,uniform_name:&str, value:i32) 
    { 
        let loc = self.get_uniform_location(uniform_name);
        unsafe {        
            gl::Uniform1i(loc, value); 
        }    
}
    // ------------------------------------------------------------------------
    pub fn set_float(&mut self,uniform_name:&str, value:f32) 
    { 
        let loc = self.get_uniform_location(uniform_name);
        unsafe {
            gl::Uniform1f(loc, value); 
        }    
    }
}   

       

   

    