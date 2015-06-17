use glium;

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

pub fn open_window() {
  use glium::{DisplayBuild, Surface};
  let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

  implement_vertex!(Vertex, position);

  // let vertex1 = Vertex { position: [-0.5, -0.5] };
  // let vertex2 = Vertex { position: [ 0.0,  0.5] };
  // let vertex3 = Vertex { position: [ 0.5, -0.25] };
  // let shape = vec![vertex1, vertex2, vertex3];
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

  let mut t = -0.5;

  loop {
    // we update `t`
    t += 0.0002;
    if t > 0.5 {
      t = -0.5;
    }

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);

    let uniforms = uniform! {
        matrix: [
          [1.0, 0.0, 0.0, 0.0],
          [0.0, 1.0, 0.0, 0.0],
          [0.0, 0.0, 1.0, 0.0],
          [ t , t, 0.0, 1.0],
          ]
    };

    target.draw(&vertex_buffer, &indices, &program, &uniforms,
        &Default::default()).unwrap();
    target.finish();

    if display.is_closed() {
      break;
    }
  }
}
