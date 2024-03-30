use std::ffi::CStr;

use glad_gl::gl;
use glfw::{Action, Context, Key, PWindow};

fn process_input(window: &mut PWindow) {
    if window.get_key(Key::Enter) == Action::Press {
        // window.set_should_close(true);
        unsafe {
            // gl::ClearColor(1.0, 0.0, 0.0, 1.0);
            // gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        // vertex_input();
    }
}

const VERTEX_SHADER: &CStr = c"#version 330 core
layout (location = 0) in vec3 aPos;
void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}";

const FRAG_SHADER: &CStr = c"#version 330 core
out vec4 FragColor;
void main()
{
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}";

fn check_compilation_status(shader: u32) {
    unsafe {
        let mut success = 0;
        let mut info_log = vec![0u8; 512];
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            gl::GetShaderInfoLog(
                shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as _,
            );
            let s = CStr::from_bytes_until_nul(&info_log);
            eprintln!("compilation failed: {:?}", s);
        }
    }
}

fn check_linking_status(program: u32) {
    unsafe {
        let mut success = 0;
        let mut info_log = vec![0u8; 512];
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            gl::GetProgramInfoLog(
                program,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as _,
            );
            let s = CStr::from_bytes_until_nul(&info_log);
            eprintln!("linking failed: {:?}", s);
        }
    }
}

fn vertex_input() -> (u32, u32, u32) {
    let shader_program = unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(
            vertex_shader,
            1,
            &VERTEX_SHADER.as_ptr() as _,
            std::ptr::null(),
        );
        gl::CompileShader(vertex_shader);
        check_compilation_status(vertex_shader);

        let frag_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(frag_shader, 1, &FRAG_SHADER.as_ptr() as _, std::ptr::null());
        gl::CompileShader(frag_shader);
        check_compilation_status(frag_shader);

        let shader_program = gl::CreateProgram();

        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, frag_shader);
        gl::LinkProgram(shader_program);
        check_linking_status(shader_program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(frag_shader);

        shader_program
    };

    #[rustfmt::skip]
    let vertices: [f32; 18] = [
        // Each line is a triangle:
        // x, y, z
        -0.5, -0.5, 0.0, // left
        0.5, -0.5, 0.0, // right
        0.0, 0.5, 0.0, // top

        0.0, -0.5, 0.0,  // left
        0.9, -0.5, 0.0,  // right
        0.45, 0.5, 0.0   // top
    ];

    let mut vbo = 0;
    let mut vao = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let nbytes = dbg!(std::mem::size_of_val(&vertices));
        gl::BufferData(
            gl::ARRAY_BUFFER,
            nbytes as _,
            vertices.as_ptr() as _,
            gl::STATIC_DRAW,
        );

        let stride = 3 * dbg!(std::mem::size_of::<f32>());
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride as i32, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // gl::UseProgram(shader_program);
        gl::BindVertexArray(0);

        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        // gl::DrawArrays(gl::TRIANGLES, 0, 3);

        eprintln!("OK");

        return (shader_program, vao, vbo);
    }
}

fn main() {
    let mut glfw = glfw::init_no_callbacks().unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, _events) = glfw
        .create_window(800, 600, "Learn OpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();

    gl::load(|e| glfw.get_proc_address_raw(e) as *const std::os::raw::c_void);

    let (shader_program, vao, vbo) = vertex_input();

    while !window.should_close() {
        process_input(&mut window);

        // unsafe {
        //     // gl::Viewport(0, 0, 800, 600);
        //     gl::ClearColor(0.2, 0.2, 0.1, 1.0);
        //     gl::Clear(gl::COLOR_BUFFER_BIT);
        // }

        unsafe {
            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }

        window.swap_buffers();
        glfw.poll_events();
    }

    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteProgram(shader_program);
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
