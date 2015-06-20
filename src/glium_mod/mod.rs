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
            let vert = maybe_vert.map( |v| Vertex{position: [v[0], v[1]]} );
            vert
        }).collect::<Vec<_>>();

    triangles
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

#[cfg(feature = "window")]
pub fn open_window() {
    use glium;
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    implement_vertex!(Vertex, position);

    let shape = octahedron(0.5);

    let vertex_buffer = glium::VertexBuffer::new(&display, shape);
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 0.0, 1.0);
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

    let draw_scene = |x: f32, y: f32| {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = uniform! {
            matrix: [
              [1.0, 0.0, 0.0, 0.0],
              [0.0, 1.0, 0.0, 0.0],
              [0.0, 0.0, 1.0, 0.0],
              [ x , y, 0.0, 1.0],
              ]
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
            &Default::default()).unwrap();
        target.finish();
    };

    for event in display.wait_events() {
        match event {
            glium::glutin::Event::Closed => {
                println!("Closing: {:?}", event);
                break;
            }
            glium::glutin::Event::MouseMoved((x, y)) => {
                println!("Mouse: ({}, {})", x, y);
                draw_scene((x as f32)/800.0, (y as f32)/600.0);
            }
            event => println!("Event: {:?}", event)
        }
    }
}
