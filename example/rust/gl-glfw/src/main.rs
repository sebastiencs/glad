use std::{ffi::CStr, sync::atomic::{AtomicBool, AtomicU8, Ordering::Relaxed}};

use glad_gl::gl;
use glfw::{Action, Context, Key, PWindow};

static MODE: AtomicU8 = AtomicU8::new(0);
static WAS_PRESSED: AtomicBool = AtomicBool::new(false);

fn process_input(window: &mut PWindow) {
    const MODES: [u32; 3] = [
        gl::LINE,
        gl::FILL,
        gl::POINT,
    ];

    let was_pressed = WAS_PRESSED.load(Relaxed);
    if window.get_key(Key::Enter) == Action::Press {
        if was_pressed {
            return
        }
        WAS_PRESSED.store(true, Relaxed);
        // window.set_should_close(true);
        unsafe {
            // gl::ClearColor(1.0, 0.0, 0.0, 1.0);
            // gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let next_mode = MODE.fetch_add(1, Relaxed) as usize;
            let next_mode = MODES[next_mode % 3];
            gl::PolygonMode(gl::FRONT_AND_BACK, next_mode);
        }
    } else {
        WAS_PRESSED.store(false, Relaxed);
    }
}

const VERTEX_SHADER: &CStr = c"#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
out vec3 ourColor;
out vec3 ourPosition;
uniform float x_offset;
void main()
{
    gl_Position = vec4(aPos.x + x_offset, aPos.y, aPos.z, 1.0);
    ourPosition = gl_Position.xyz;
    ourColor = aColor;
}";

const FRAG_SHADER: &CStr = c"#version 330 core
out vec4 FragColor;
in vec3 ourColor;
in vec3 ourPosition;
void main()
{
    FragColor = vec4(ourPosition, 1.0f);
    // FragColor = vec4(ourColor, 1.0f);
    // FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}";

const FRAG2_SHADER: &CStr = c"#version 330 core
out vec4 FragColor;
uniform vec4 ourColor;
void main()
{
    // FragColor = vec4(1.0f, 0.0f, 0.0f, 0.9f);
    FragColor = ourColor;
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
        // positions       // colors
        0.5, -0.5, 0.0,    1.0, 0.0, 0.0,   // bottom right
        -0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   // bottom left
        0.0,  0.5, 0.0,    0.0, 0.0, 1.0    // top
        // // positions       // colors
        // 0.5, 0.5, 0.0,    1.0, 0.0, 0.0,   // bottom right
        // -0.5, 0.5, 0.0,   0.0, 1.0, 0.0,   // bottom left
        // 0.0,  -0.5, 0.0,    0.0, 0.0, 1.0    // top
    ];

    let mut vbo1 = 0;
    let mut vao1 = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao1);
        gl::GenBuffers(1, &mut vbo1);

        gl::BindVertexArray(vao1);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo1);
        let nbytes = dbg!(std::mem::size_of_val(&vertices));
        gl::BufferData(
            gl::ARRAY_BUFFER,
            nbytes as _,
            vertices.as_ptr() as _,
            gl::STATIC_DRAW,
        );

        // position
        let stride = 6 * dbg!(std::mem::size_of::<f32>());
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride as i32, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        // color
        let stride = 6 * dbg!(std::mem::size_of::<f32>());
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride as i32, (3 * std::mem::size_of::<f32>()) as _);
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // gl::UseProgram(shader_program);
        gl::BindVertexArray(0);

        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        // gl::DrawArrays(gl::TRIANGLES, 0, 3);

        eprintln!("OK");

        // return (shader_program, vao1, vbo1);
    }

    return (shader_program, vao1, vbo1);
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

    let (shader_program, vao1, vbo1) = vertex_input();

    while !window.should_close() {
        process_input(&mut window);

        let time_value = glfw.get_time();
        let green_value = ((time_value.sin()) / 2.0) + 0.5;

        unsafe {
            // let vertex_color_location = gl::GetUniformLocation(shader_program, c"ourColor".as_ptr() as _);
            // assert_ne!(vertex_color_location, -1);

            let x_offset_location = gl::GetUniformLocation(shader_program, c"x_offset".as_ptr() as _);
            assert_ne!(x_offset_location, -1);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao1);
            gl::Uniform1f(x_offset_location, 0.2);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            // gl::UseProgram(shader_program2);
            // gl::BindVertexArray(vao2);
            // gl::Uniform4f(vertex_color_location, 0.0, green_value as _, 0.0, 1.0);
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers();
        glfw.poll_events();
    }

    let mut nr_attributes = 0;

    unsafe {
        gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut nr_attributes);
        gl::DeleteVertexArrays(1, &vao1);
        gl::DeleteBuffers(1, &vbo1);
        // gl::DeleteBuffers(1, &vbo2);
        gl::DeleteProgram(shader_program);
        // gl::DeleteProgram(shader_program2);
    }

    dbg!(nr_attributes);
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
