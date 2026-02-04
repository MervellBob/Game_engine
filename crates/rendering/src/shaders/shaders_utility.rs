use std::fs::File;
use asset::asset::Asset;
use asset::text::Text;
use std::collections::HashMap;
use std::ffi::CString;
use tracing::{Level, error, info, span};


/// **Shader**
///
/// A utility structure for managing OpenGL shaders.  
/// This struct **encapsulates shader compilation and program linking**, 
/// and provides convenient methods for **setting and caching uniform variables**.
///
/// # Purpose
/// The `Shader` struct is designed to simplify the workflow of OpenGL shader management:
/// - Compile vertex and fragment shaders from source or file
/// - Link them into a program
/// - Track uniform locations efficiently to avoid repeated `glGetUniformLocation` calls
///
/// # Examples
/// ```ignore
/// // Create a shader program from predefined shader files
/// let shader = Shader::new();
/// shader.use_program();
/// shader.set_float("someUniform", 0.5);
/// ```
///
/// # Panics
/// - Panics if shader compilation fails, providing the compilation log.
/// - Panics if program linking fails, providing the linking log.
///
/// # Notes
/// - Uniform locations are **cached internally** for performance.
/// - Intended for use with OpenGL 3.3 core profile.
/// - The struct does **not handle shader reloading**; create a new instance for updated shaders.
pub struct Shader {
    pub program_id:u32, 
    uniforms: HashMap<String, i32>,

}

impl Shader {

    /// **Creates a new `Shader` instance.**
    ///
    /// This constructor compiles the vertex and fragment shaders located in 
    /// the `src/shaders/test_shader/` directory and links them into an OpenGL shader program.  
    /// The resulting `Shader` object holds the `program_id` and initializes a uniform cache.
    ///
    /// # Behavior
    /// - Loads shader source files from the project directory using `CARGO_MANIFEST_DIR`.
    /// - Compiles the vertex shader (`vertex_1.glsl`) and fragment shader (`fragment_1.glsl`).
    /// - Links the shaders into a program, storing the resulting `program_id`.
    /// - Initializes an empty `HashMap` to cache uniform locations.
    ///
    /// # Panics
    /// - Panics if a shader file cannot be opened.
    /// - Panics if shader compilation fails, providing the compilation log.
    /// - Panics if program linking fails, providing the link log.
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        let span = span!(Level::INFO, "Creating new shader program");
        let _enter = span.enter();

        let vertex_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"),vertex_path);
        let fragment_path: String = format!("{}/{}", env!("CARGO_MANIFEST_DIR"),fragment_path);

        let vertex_shader = Self::compile_shader(&vertex_path,gl::VERTEX_SHADER);
        let fragment_shader = Self::compile_shader(&fragment_path,gl::FRAGMENT_SHADER);

        let shader_program = Self::create_program(vertex_shader,fragment_shader);

        Self { 
            program_id:shader_program,
            uniforms: HashMap::new()
        }
    }

    /// **Compiles a shader from a source file.**
    ///
    /// This function reads a GLSL shader source file from the provided `shader_path` and compiles it 
    /// into an OpenGL shader object of the specified `shader_type` (e.g., `gl::VERTEX_SHADER` or `gl::FRAGMENT_SHADER`).
    ///
    /// # Returns
    /// Returns the `u32` ID of the compiled shader object.
    ///
    /// # Panics
    /// - Panics if the shader file cannot be opened or read.
    /// - Panics if the file contains invalid UTF-8.
    /// - Panics if OpenGL fails to create the shader object.
    /// - Panics if shader compilation fails, providing the shader compilation log.
    ///
    /// # Safety
    /// This function uses raw OpenGL calls (`gl::CreateShader`, `gl::ShaderSource`, `gl::CompileShader`) 
    /// and assumes a valid OpenGL context is active.
    fn compile_shader(shader_path: &str, shader_type: u32) -> u32 {
        let shader_type_str = match shader_type {
            gl::VERTEX_SHADER => "VERTEX",
            gl::FRAGMENT_SHADER => "FRAGMENT",
            gl::GEOMETRY_SHADER => "GEOMETRY", // si tu en utilises
            _ => "UNKNOWN",
        };


        let span = tracing::span!(tracing::Level::INFO, "Compiling shader", path = shader_path, shader_type = shader_type_str);
        let _enter = span.enter();

        // Attempt to open the file
        let file = match File::open(shader_path) {
            Ok(f) => {
                info!("Shader file opened successfully");
                f
            },
            Err(e) => {
                error!("Unable to open shader file {}: {}", shader_path, e);
                panic!();
            }
        };

        let mut file: Text = Text::new(file);
        file.read_raw();
        let shader_content = match String::from_utf8(file.contents_raw) {
            Ok(s) => {
                info!("Shader content accessed successfully");
                s
            }
            Err(e) => {
                error!("Found invalid UTF-8 in shader file{}: {}", shader_path, e);
                panic!();
            }
        };

        unsafe {
            let shader = gl::CreateShader(shader_type);
            if shader == 0 {
                error!("Failed to create OpenGL shader object");
                panic!();
            } else {
                info!(shader = shader, shader_type = shader_type_str, "Shader object created");
            }


            gl::ShaderSource(
                shader,
                1,
                &(shader_content.as_bytes().as_ptr().cast()),
                &(shader_content.len().try_into().unwrap()),
            );

            gl::CompileShader(shader);
            Self::check_compile_error(shader, "NOT PROGRAM");
            info!(shader = shader,shader_type = shader_type_str, "Shader compiled successfully");
            shader
        }
    }

    /// **Creates and links an OpenGL shader program from vertex and fragment shaders.**
    ///
    /// This function takes the IDs of a compiled vertex shader and fragment shader, attaches them
    /// to a new OpenGL program, links the program, and validates its compilation status.  
    /// After linking, the individual shaders are deleted since they are no longer needed.
    ///
    /// # Returns
    /// Returns the OpenGL program ID (`u32`) of the linked shader program.
    ///
    /// # Panics
    /// - Panics if the program linking fails, providing the program link log via `check_compile_error`.
    ///
    /// # Safety
    /// - Requires a valid OpenGL context to be active.
    /// - Uses raw OpenGL calls (`gl::CreateProgram`, `gl::AttachShader`, `gl::LinkProgram`, `gl::UseProgram`, `gl::DeleteShader`).
    ///
    /// # Notes
    /// - The function deletes the attached shaders after linking to free GPU resources.
    fn create_program(vertex_shader:u32,fragment_shader:u32) -> u32 {  
        unsafe {
            let shader_program = gl::CreateProgram();
            let span = tracing::span!(tracing::Level::INFO, "Linking program", shader_program = shader_program);
            let _enter = span.enter();

            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);
            Self::check_compile_error(shader_program, "PROGRAM");
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            shader_program
        }
    }
    
    /// **Checks for compilation or linking errors in shaders or shader programs.**
    ///
    /// This utility function queries OpenGL to verify whether a shader or a shader program
    /// compiled/linked successfully. If an error occurred, it retrieves the corresponding log
    /// and panics with a detailed message.
    ///
    /// # Parameters
    /// - `shader`: The OpenGL ID of the shader or shader program.
    /// - `shader_type`: A string indicating the type; `"PROGRAM"` for programs, otherwise assumed to be a shader.
    ///
    /// # Behavior
    /// - For shader programs (`shader_type == "PROGRAM"`):
    ///     - Checks `LINK_STATUS` to determine if linking succeeded.
    ///     - Retrieves the program info log if linking failed.
    /// - For individual shaders (vertex, fragment, etc.):
    ///     - Checks `COMPILE_STATUS` to determine if compilation succeeded.
    ///     - Retrieves the shader info log if compilation failed.
    ///
    /// # Panics
    /// - Panics if the shader or program failed to compile or link, with the OpenGL error log included.
    ///
    /// # Safety
    /// - Requires a valid OpenGL context.
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
                        error!("Program link error:\n {}", log);
                        panic!();
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
                        error!("Shader compile error:\n {}", log);
                        panic!();
                    }
                }
            }
            
        }

    }

    /// **Retrieves the location of a uniform variable in the shader program.**
    ///
    /// This function checks whether the requested uniform's location is already cached
    /// in the internal `HashMap`. If it is, the cached value is returned. Otherwise,
    /// it queries OpenGL using `gl::GetUniformLocation`, stores the result in the cache,
    /// and returns the location.
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
    
    /// Activates the shader program for subsequent rendering commands.
    pub fn use_program(&self) 
    { 
        unsafe {
            gl::UseProgram(self.program_id); 
        }
    }

    /// Sets a boolean uniform in the shader program.
    pub fn set_bool(&mut self,uniform_name:&str, value:bool)
    {         
        let loc = self.get_uniform_location(uniform_name);
        unsafe {
            gl::Uniform1i(loc, value as i32);
        }
    }
    
    /// Sets an integer uniform in the shader program.
    pub fn set_int(&mut self,uniform_name:&str, value:i32) 
    { 
        let loc = self.get_uniform_location(uniform_name);
        unsafe {        
            gl::Uniform1i(loc, value); 
        }    
    }

    /// Sets a floating-point uniform in the shader program.
    pub fn set_float(&mut self,uniform_name:&str, value:f32) 
    { 
        let loc = self.get_uniform_location(uniform_name);
        unsafe {
            gl::Uniform1f(loc, value); 
        }    
    }
}   

       

   

    