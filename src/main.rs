#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

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
    let quality_uniform = gl.get_uniform_location(program, c"quality");

    gl.bind_vertex_array(vao);

    let mut old_pointer_x: f64 = window.pointer_coordinates().0;
    let mut old_pointer_y: f64 = window.pointer_coordinates().1;
    let mut center_real: f32 = 0.0;
    let mut center_imaginary: f32 = 0.0;
    let mut quality: u8 = 1;
    let mut up_pressed = false;
    let mut down_pressed = false;
    loop {
        let (width, height) = gl.get_viewport();
        let (pointer_x, pointer_y) = window.pointer_coordinates();
        let (movement_x, movement_y) =
            if !window.left_button() || old_pointer_x == 0.0 && old_pointer_y == 0.0 {
                (0.0, 0.0)
            } else {
                (pointer_x - old_pointer_x, pointer_y - old_pointer_y)
            };
        let zoom = 10.0f32.powf(window.total_scroll() as f32 / 100.0);
        center_real -= movement_x as f32 / width as f32 / zoom * 2.0;
        center_imaginary += movement_y as f32 / height as f32 / zoom * 2.0;
        if window.up_key() && !up_pressed {
            up_pressed = true;
            quality += 1;
        } else if !window.up_key() && up_pressed {
            up_pressed = false;
        }
        if window.down_key() && !down_pressed {
            down_pressed = true;
            quality = u8::max(quality - 1, 1);
        } else if !window.down_key() && down_pressed {
            down_pressed = false;
        }

        gl.uniform_2i(viewport_uniform, width, height);
        gl.uniform_1f(zoom_uniform, zoom);
        gl.uniform_2f(center_uniform, center_real, center_imaginary);
        gl.uniform_1i(quality_uniform, i32::from(quality));
        gl.draw_elements(&ELEMENTS);
        window.swap_buffers().unwrap();
        window.check_events().unwrap();

        old_pointer_x = pointer_x;
        old_pointer_y = pointer_y;
    }
}
