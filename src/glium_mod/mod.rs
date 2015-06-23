fn octahedron(length: f32) -> Vec<Vertex> {
    let ratio = 0.1;
    let top_length = ratio * length;
    let thel: f32 = (2.0f32).sqrt() * top_length / 2.0;

    let vertices = [
        [  0.0,   0.0,        0.0],
        [-thel,  thel, top_length],
        [ thel,  thel, top_length],
        [ thel, -thel, top_length],
        [-thel, -thel, top_length],
        [  0.0,   0.0,     length]
      ];


    let triangle_indices = [
            0, 1, 2,
            0, 2, 3,
            0, 3, 4,
            0, 4, 1,

            5, 2, 1,
            5, 3, 2,
            5, 4, 3,
            5, 1, 4usize
        ];

    let triangles =
        triangle_indices.iter().flat_map( |idx| {
            let maybe_vert = vertices.get(*idx);
            let vert = maybe_vert.map( |v| Vertex{position: [v[0], v[1], v[2]]} );
            vert
        }).collect::<Vec<_>>();

    triangles
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}

use nalgebra as na;
use nalgebra::{Mat4, Iso3, Rot3, Vec3};

#[cfg(feature = "window")]
pub fn open_window() {
    use glium;
    use glium::{DisplayBuild, Surface};
    use glium::glutin::{Event, ElementState, MouseButton};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    implement_vertex!(Vertex, position);

    let shape = octahedron(0.5);

    let vertex_buffer = glium::VertexBuffer::new(&display, shape);
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec3 position;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
          color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let draw_scene = |window_state: &WindowState, last_window_state: &WindowState| {
        let (x, y) = window_state.scaled_mouse_position;
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let iso = Iso3::new_with_rotmat(
                    Vec3::new(x, y, 0.0),
                    Rot3::new(Vec3::new(0f32, 0.0, 0.0)));

        let homo: Mat4<_> = na::to_homogeneous(&iso);

        let uniforms = uniform! { matrix: *homo.as_array() };

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
            &Default::default()).unwrap();
        target.finish();
    };

    let last_window_state = WindowState {
        scaled_mouse_position: (1337.0, 1337.0),
        last_scaled_mouse_position: (1338.0, 1338.0),
        is_left_drag: false
    };
    for event in display.wait_events() {
        let mut window_state = last_window_state.clone();

        match event {
            Event::Closed => {
                println!("Closing: {:?}", event);
                break;
            }
            Event::MouseMoved((x, y)) => {
                //println!("Mouse: ({}, {})", x, y);
                let size = display.get_window().and_then( |win|
                    win.get_inner_size()).unwrap_or((2880, 1800));
                let scaled_mouse = scale_mouse_position(size, (x, y));
                //println!("scaled_mouse: {:?}", scaled_mouse);
                window_state.scaled_mouse_position = scaled_mouse;
            }
            Event::MouseInput(action, button) => {
                match (action, button) {
                    (ElementState::Pressed,  MouseButton::Left) => { window_state.is_left_drag = true; }
                    (ElementState::Released, MouseButton::Left) => { window_state.is_left_drag = false; }
                    _ => {}
                }
            }
            event => println!("Event: {:?}", event)
            //_ => {}
        }

        draw_scene(&window_state, &last_window_state);
    }
}

#[derive(Clone)]
struct WindowState {
    scaled_mouse_position: (f32, f32),
    last_scaled_mouse_position: (f32, f32),
    is_left_drag: bool
}


fn scale_mouse_position((wi, hi): (u32, u32), (xi, yi): (i32, i32)) -> (f32, f32) {
    let (w, h, x, y) = (wi as f32, hi as f32, xi as f32, yi as f32);
    (x/(w*2.0) - 1.0, -y/(h*2.0))
}
