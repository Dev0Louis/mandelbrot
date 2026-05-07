#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]

use crate::{
    gl::{GL, Triangle},
    window::Window,
};

mod gl;
mod math;
mod window;

#[repr(C)]
struct Vertex {
    x: f32,
    y: f32,
}
static VERTICES: [Vertex; 4] = [
    Vertex { x: 1.0, y: 1.0 },
    Vertex { x: 1.0, y: -1.0 },
    Vertex { x: -1.0, y: -1.0 },
    Vertex { x: -1.0, y: 1.0 },
];
static ELEMENTS: [Triangle<u8>; 2] = [Triangle(0, 1, 3), Triangle(1, 2, 3)];
const VERTEX_SHADER_SOURCE: &str = include_str!("shader/vertex.glsl");
const FRAGMENT_SHADER_SOURCE: &str = include_str!("shader/fragment.glsl");

fn main() {
    let window = Window::create().unwrap();
    let gl = window.load_gl();

    gl.enable(GL::DEBUG_OUTPUT);
    gl.debug_message_callback(|_, _, _, _, message| eprintln!("OpenGL Debug: {message}"));

    let vbo = gl.create_buffer();
    gl.named_buffer_data(vbo, &VERTICES, GL::STATIC_DRAW);
    let vao = gl.create_vertex_array();
    unsafe {
        gl.vertex_array_vertex_buffer(vao, 0, vbo, 0, std::mem::size_of::<Vertex>() as u32);
    }
    gl.enable_vertex_array_attrib(vao, 0);
    gl.vertex_array_attrib_format(vao, 0, 2, GL::FLOAT, false, 0);
    gl.vertex_array_attrib_binding(vao, 0, 0);

    let ebo = gl.create_buffer();
    gl.named_buffer_data(ebo, &ELEMENTS, GL::STATIC_DRAW);
    gl.vertex_array_element_buffer(vao, ebo);

    let vertex_shader = gl.create_shader(GL::VERTEX_SHADER);
    gl.shader_source(vertex_shader, VERTEX_SHADER_SOURCE);
    gl.compile_shader(vertex_shader);
    let fragment_shader = gl.create_shader(GL::FRAGMENT_SHADER);
    gl.shader_source(fragment_shader, FRAGMENT_SHADER_SOURCE);
    gl.compile_shader(fragment_shader);
    let program = gl.create_program();
    gl.attach_shader(program, vertex_shader);
    gl.attach_shader(program, fragment_shader);
    gl.link_program(program);
    gl.use_program(program);

    let viewport_uniform = gl.get_uniform_location(program, c"viewport");
    let zoom_uniform = gl.get_uniform_location(program, c"zoom");
    let center_uniform = gl.get_uniform_location(program, c"center");

    gl.bind_vertex_array(vao);
    let mut i = 1.0;
    loop {
        let (width, height) = gl.get_viewport();
        gl.uniform_2i(viewport_uniform, width, height);
        gl.uniform_1f(zoom_uniform, i);
        gl.uniform_2f(center_uniform, -0.75, 0.1);
        //i *= 1.01;

        gl.draw_elements(&ELEMENTS);
        window.swap_buffers().unwrap();
        window.check_events().unwrap();
    }
}
