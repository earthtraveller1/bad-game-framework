use crate::math;
use glad_gl::gl;
use stb::image::*;

const MAX_TEXTURES: u32 = 32;

pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

impl Color {
    fn to_vec(&self) -> math::Vector4<f32> {
        math::Vector4 {
            x: self.red,
            y: self.green,
            z: self.blue,
            w: self.alpha,
        }
    }
}

pub struct Renderer2D {
    vertex_array: VertexArray,
    vertex_buffer: Buffer,
    index_buffer: Buffer,

    textures: [Option<InternalTexture>; 32],

    vertices: Vec<Vertex2D>,
    quads_to_draw: u32,
    max_quads: u32,
    next_texture_slot: u32,

    shader_program: ShaderProgram,
}

impl Renderer2D {
    pub fn new(max_quads: usize, window_width: f32, window_height: f32) -> Renderer2D {
        let shader_program = ShaderProgram::new(
            "assets/shaders/2d_renderer_basic.vs",
            "assets/shaders/2d_renderer_basic.fs",
        );
        shader_program.use_program();

        for i in 0..32 {
            shader_program.set_uniform_1i(format!("texture_samplers[{}]", i).as_str(), i);
        }

        let projection = math::orthographic(0.0, window_width, 0.0, window_height, 1.0, 0.0);

        shader_program.set_unifrom_matrix_4f("projection", &projection);

        let vertex_array = VertexArray::new();
        vertex_array.bind();

        let vertices = Vec::with_capacity(max_quads * 4);

        let vertex_buffer = Buffer::new_empty(
            (max_quads * std::mem::size_of::<Vertex2D>() * 4)
                .try_into()
                .unwrap(),
            BufferType::Vertex,
            BufferUsage::Dynamic,
        );
        vertex_buffer.bind();

        let mut indices: Vec<u32> = Vec::with_capacity(max_quads * 6);

        for i in 0..max_quads {
            indices.push((i * 4 + 0).try_into().unwrap());
            indices.push((i * 4 + 1).try_into().unwrap());
            indices.push((i * 4 + 2).try_into().unwrap());
            indices.push((i * 4 + 0).try_into().unwrap());
            indices.push((i * 4 + 3).try_into().unwrap());
            indices.push((i * 4 + 2).try_into().unwrap());
        }

        let index_buffer = Buffer::new(indices, BufferType::Index, BufferUsage::Static);
        index_buffer.bind();

        vertex_array.create_vertex_attribute(
            0,
            2,
            OpenGLType::Float,
            std::mem::size_of::<Vertex2D>().try_into().unwrap(),
            0,
        );

        vertex_array.create_vertex_attribute(
            1,
            2,
            OpenGLType::Float,
            std::mem::size_of::<Vertex2D>().try_into().unwrap(),
            2 * std::mem::size_of::<f32>(),
        );

        vertex_array.create_vertex_attribute(
            2,
            4,
            OpenGLType::Float,
            std::mem::size_of::<Vertex2D>().try_into().unwrap(),
            4 * std::mem::size_of::<f32>(),
        );

        vertex_array.create_vertex_attribute(
            3,
            1,
            OpenGLType::Float,
            std::mem::size_of::<Vertex2D>().try_into().unwrap(),
            8 * std::mem::size_of::<f32>(),
        );

        // Too lazy to implement the copy trait for Texture
        let textures = [
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
            Option::<InternalTexture>::None,
        ];

        return Renderer2D {
            vertex_array,
            vertex_buffer,
            index_buffer,
            textures,
            vertices,
            quads_to_draw: 0,
            max_quads: max_quads.try_into().unwrap(),
            shader_program,
            next_texture_slot: 0,
        };
    }

    pub fn load_texture(&mut self, image_file_path: &str) -> f32 {
        if self.next_texture_slot < MAX_TEXTURES {
            self.textures[self.next_texture_slot as usize] =
                Some(InternalTexture::new(image_file_path));
            self.next_texture_slot += 1;

            return (self.next_texture_slot - 1) as f32;
        } else {
            return 0.0;
        }
    }

    pub fn begin(&mut self) {
        self.quads_to_draw = 0;
        self.vertices.clear();
    }

    pub fn draw_quad_v(
        &mut self,
        position: &math::Vector2<f32>,
        size: &math::Vector2<f32>,
        color: &math::Vector4<f32>,
        texture_id: f32,
    ) {
        self.quads_to_draw += 1;

        // If the client made more draw calls than what was allocated, then do
        // nothing.
        if self.quads_to_draw > self.max_quads {
            self.quads_to_draw -= 1;
            return;
        }

        use math::Vector2;

        let vertex = Vertex2D {
            position: Vector2::<f32> {
                x: position.x + size.x,
                y: position.y,
            },
            uv: Vector2::<f32> { x: 1.0, y: 0.0 },
            color: color.clone(),
            texture: texture_id,
        };
        self.vertices.push(vertex);

        let vertex = Vertex2D {
            position: Vector2::<f32> {
                x: position.x + size.x,
                y: position.y + size.y,
            },
            uv: Vector2::<f32> { x: 1.0, y: 1.0 },
            color: color.clone(),
            texture: texture_id,
        };
        self.vertices.push(vertex);

        let vertex = Vertex2D {
            position: Vector2::<f32> {
                x: position.x,
                y: position.y + size.y,
            },
            uv: Vector2::<f32> { x: 0.0, y: 1.0 },
            color: color.clone(),
            texture: texture_id,
        };
        self.vertices.push(vertex);

        let vertex = Vertex2D {
            position: Vector2::<f32> {
                x: position.x,
                y: position.y,
            },
            uv: Vector2::<f32> { x: 0.0, y: 0.0 },
            color: color.clone(),
            texture: texture_id,
        };
        self.vertices.push(vertex);
    }

    pub fn draw_quad(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: &Color,
        texture_id: f32,
    ) {
        self.quads_to_draw += 1;

        // If the client made more draw calls than what was allocated, then do
        // nothing.
        if self.quads_to_draw > self.max_quads {
            self.quads_to_draw -= 1;
            return;
        }

        use math::Vector2;

        let vertex = Vertex2D {
            position: Vector2::<f32> { x: x + width, y: y },
            uv: Vector2::<f32> { x: 1.0, y: 0.0 },
            color: color.to_vec(),
            texture: texture_id,
        };
        self.vertices.push(vertex);

        let vertex = Vertex2D {
            position: Vector2::<f32> {
                x: x + width,
                y: y + height,
            },
            uv: Vector2::<f32> { x: 1.0, y: 1.0 },
            color: color.to_vec(),
            texture: texture_id,
        };
        self.vertices.push(vertex);

        let vertex = Vertex2D {
            position: Vector2::<f32> {
                x: x,
                y: y + height,
            },
            uv: Vector2::<f32> { x: 0.0, y: 1.0 },
            color: color.to_vec(),
            texture: texture_id,
        };
        self.vertices.push(vertex);

        let vertex = Vertex2D {
            position: Vector2::<f32> { x: x, y: y },
            uv: Vector2::<f32> { x: 0.0, y: 0.0 },
            color: color.to_vec(),
            texture: texture_id,
        };
        self.vertices.push(vertex);
    }

    pub fn end(&self) {
        self.vertex_array.bind();
        self.vertex_buffer.set_sub_data(0, &self.vertices);
        self.index_buffer.bind();

        let quad_count: i32 = self.quads_to_draw.try_into().unwrap();

        for i in 0..32 {
            if let Some(texture) = &self.textures[i] {
                InternalTexture::set_active_texture(i.try_into().unwrap());
                texture.bind();
            }
        }

        self.shader_program.use_program();

        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                quad_count * 6,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}

// I will only add the types that I will be using
enum OpenGLType {
    Float,
    _Short,
}

struct VertexArray {
    handle: u32,
}

impl VertexArray {
    fn new() -> VertexArray {
        let mut handle = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut handle);
        }

        return VertexArray { handle: handle };
    }

    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.handle);
        }
    }

    fn _unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    fn create_vertex_attribute(
        &self,
        index: u32,
        size: u8,
        attribute_type: OpenGLType,
        stride: i32,
        offset: usize,
    ) {
        unsafe {
            self.bind();
            gl::VertexAttribPointer(
                index,
                size.try_into().unwrap(),
                match attribute_type {
                    OpenGLType::Float => gl::FLOAT,
                    OpenGLType::_Short => gl::SHORT,
                },
                gl::FALSE,
                stride,
                offset as *const std::ffi::c_void,
            );
            gl::EnableVertexAttribArray(index);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.handle);
        }
    }
}

enum BufferType {
    Vertex,
    Index,
}

// Once again, only included the ones that I will use.
enum BufferUsage {
    Static,
    Dynamic,
}

struct Buffer {
    handle: u32,
    buffer_type: u32,
}

impl Buffer {
    fn new<T>(data: Vec<T>, buffer_type: BufferType, buffer_usage: BufferUsage) -> Buffer {
        let mut handle = 0;
        let buffer_type = match buffer_type {
            BufferType::Vertex => gl::ARRAY_BUFFER,
            BufferType::Index => gl::ELEMENT_ARRAY_BUFFER,
        };

        unsafe {
            gl::GenBuffers(1, &mut handle);
            gl::BindBuffer(buffer_type, handle);
            gl::BufferData(
                buffer_type,
                (data.len() * std::mem::size_of::<T>()).try_into().unwrap(),
                data.as_ptr() as *const std::ffi::c_void,
                match buffer_usage {
                    BufferUsage::Static => gl::STATIC_DRAW,
                    BufferUsage::Dynamic => gl::DYNAMIC_DRAW,
                },
            );

            gl::BindBuffer(buffer_type, 0);
        }

        return Buffer {
            handle,
            buffer_type,
        };
    }

    /// Creates a new empty buffer
    fn new_empty(size: isize, buffer_type: BufferType, buffer_usage: BufferUsage) -> Buffer {
        let mut handle = 0;
        let buffer_type = match buffer_type {
            BufferType::Vertex => gl::ARRAY_BUFFER,
            BufferType::Index => gl::ELEMENT_ARRAY_BUFFER,
        };

        unsafe {
            gl::GenBuffers(1, &mut handle);
            gl::BindBuffer(buffer_type, handle);
            gl::BufferData(
                buffer_type,
                size,
                std::ptr::null(),
                match buffer_usage {
                    BufferUsage::Static => gl::STATIC_DRAW,
                    BufferUsage::Dynamic => gl::DYNAMIC_DRAW,
                },
            );

            gl::BindBuffer(buffer_type, 0);
        }

        return Buffer {
            handle,
            buffer_type,
        };
    }

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.buffer_type, self.handle);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.buffer_type, 0);
        }
    }

    fn set_sub_data<T>(&self, offset: isize, data: &Vec<T>) {
        self.bind();

        unsafe {
            gl::BufferSubData(
                self.buffer_type,
                offset,
                (data.len() * std::mem::size_of::<T>()).try_into().unwrap(),
                data.as_ptr() as *const std::ffi::c_void,
            );
        }

        self.unbind();
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        self.unbind();

        unsafe {
            gl::DeleteBuffers(1, &self.handle);
        }
    }
}

fn create_shader(source_path: &str, shader_type: u32) -> u32 {
    let source = std::fs::read_to_string(source_path);
    let source = match source {
        Ok(source) => source,
        Err(e) => {
            eprintln!(
                "[ERROR]: Failed to load file {}: {}",
                source_path,
                e.to_string()
            );
            "".to_string()
        }
    };

    let shader;

    unsafe {
        shader = gl::CreateShader(shader_type);
        let source = std::ffi::CString::new(source.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
    }

    let mut success = 0;

    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    }

    if success != gl::TRUE.into() {
        unsafe {
            let mut error_log_length = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut error_log_length);
            let mut error_log = Vec::with_capacity(error_log_length as usize);
            error_log.set_len((error_log_length as usize) - 1);
            gl::GetShaderInfoLog(
                shader,
                error_log_length,
                &mut error_log_length,
                error_log.as_mut_ptr() as *mut gl::types::GLchar,
            );
            eprintln!(
                "[ERROR]: Failed to link a shader program:\n{}\n\n",
                std::str::from_utf8(&error_log).unwrap()
            );
        }
    }

    return shader;
}

struct ShaderProgram {
    handle: u32,
}

impl ShaderProgram {
    fn new(vertex_source_path: &str, fragment_source_path: &str) -> ShaderProgram {
        let vertex = create_shader(vertex_source_path, gl::VERTEX_SHADER);
        let fragment = create_shader(fragment_source_path, gl::FRAGMENT_SHADER);

        let handle;

        unsafe {
            handle = gl::CreateProgram();
            gl::AttachShader(handle, vertex);
            gl::AttachShader(handle, fragment);
            gl::LinkProgram(handle);

            // We can delete the shaders once they're linked to the program.
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
        }

        let mut success = 0;

        unsafe {
            gl::GetProgramiv(handle, gl::LINK_STATUS, &mut success);
        }

        if success != gl::TRUE.into() {
            unsafe {
                let mut error_log_length = 0;
                gl::GetProgramiv(handle, gl::INFO_LOG_LENGTH, &mut error_log_length);
                let mut error_log = Vec::with_capacity(error_log_length as usize);
                error_log.set_len((error_log_length as usize) - 1);
                gl::GetProgramInfoLog(
                    handle,
                    error_log_length,
                    &mut error_log_length,
                    error_log.as_mut_ptr() as *mut gl::types::GLchar,
                );
                eprintln!(
                    "[ERROR]: Failed to link a shader program:\n{}\n\n",
                    std::str::from_utf8(&error_log).unwrap()
                );
            }
        }

        return ShaderProgram { handle: handle };
    }

    fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.handle);
        }
    }

    fn set_uniform_1i(&self, name: &str, value: i32) {
        let mut name = name.as_bytes().to_vec();
        name.push(0);

        unsafe {
            let location = gl::GetUniformLocation(self.handle, name.as_ptr() as *const i8);
            gl::Uniform1i(location, value);
        }
    }

    fn set_unifrom_matrix_4f(&self, name: &str, value: &math::Matrix4<f32>) {
        let mut name = name.as_bytes().to_vec();
        name.push(0);

        unsafe {
            let location = gl::GetUniformLocation(self.handle, name.as_ptr() as *const i8);
            gl::UniformMatrix4fv(location, 1, gl::TRUE, value.as_ptr());
        }
    }

    // I'll add the uniform functions when I need them.
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::UseProgram(0);
            gl::DeleteProgram(self.handle);
        }
    }
}

struct InternalTexture {
    handle: u32,
}

impl InternalTexture {
    fn new(image_path: &str) -> InternalTexture {
        let mut handle: u32 = 0;

        unsafe {
            gl::GenTextures(1, &mut handle);
            gl::BindTexture(gl::TEXTURE_2D, handle);

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR.try_into().unwrap(),
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR_MIPMAP_LINEAR.try_into().unwrap(),
            );

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_EDGE.try_into().unwrap(),
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_EDGE.try_into().unwrap(),
            );
        }

        let mut image_file = std::fs::File::open(image_path).unwrap();
        let (image_info, image_data) =
            stbi_load_from_reader(&mut image_file, Channels::Default).unwrap();

        let image_format = match image_info.components {
            1 => gl::RED,
            2 => gl::RG,
            3 => gl::RGB,
            4 => gl::RGBA,
            _ => gl::RGB,
        };

        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                image_format.try_into().unwrap(),
                image_info.width,
                image_info.height,
                0,
                image_format,
                gl::UNSIGNED_BYTE,
                image_data.into_vec().as_ptr() as *const std::ffi::c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        return InternalTexture { handle };
    }

    fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
        }
    }

    fn unbind() {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    fn set_active_texture(texture_unit: u32) {
        unsafe { gl::ActiveTexture(gl::TEXTURE0 + texture_unit) }
    }
}

impl Drop for InternalTexture {
    fn drop(&mut self) {
        InternalTexture::unbind();

        unsafe {
            gl::DeleteTextures(1, &self.handle);
        }
    }
}
#[repr(C)]
struct Vertex2D {
    position: math::Vector2<f32>,
    uv: math::Vector2<f32>,
    color: math::Vector4<f32>,
    texture: f32,
}
