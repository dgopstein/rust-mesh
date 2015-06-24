use std;

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
            let vert = maybe_vert.map( |v| Vertex{position: [v[0], v[1], v[2], 1.0]} );
            vert
        }).collect::<Vec<_>>();

    triangles
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 4],
}

use nalgebra as na;
use nalgebra::{Mat4, Iso3, Rot3, Vec3};

use glium::draw_parameters::LinearBlendingFactor::*;

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

    // let vertex_shader_src = r#"
    //     #version 140
    //
    //     in vec4 position;
    //
    //     uniform mat4 matrix;
    //
    //     void main() {
    //         gl_Position = matrix * position;
    //     }
    // "#;
    //
    // let fragment_shader_src = r#"
    //     #version 140
    //
    //     out vec4 color;
    //
    //     void main() {
    //       color = vec4(1.0, 0.5, 0.0, 0.1);
    //     }
    // "#;

    let vertex_shader_src = r#"
    #version 330

    layout(location = 0) in vec4 position;

    uniform mat4 matrix;

    out Data
    {
        vec4 position;
    } vdata;

    void main()
    {
        vdata.position = matrix * position;
    }
    "#;

    let fragment_shader_src = r#"
    #version 330

    in Data
    {
        noperspective in vec3 dist;
    } gdata;

    out vec4 outputColor;

    uniform sampler2D tex;

    const vec4 wireframeColor = vec4(1.0f, 0.5f, 0.5f, 1.0f);
    const vec4 fillColor = vec4(1.0f, 1.0f, 1.0f, 0.0f);

    void main()
    {
        float d = min(gdata.dist.x, min(gdata.dist.y, gdata.dist.z));
        float I = exp2(-2*d*d);
        outputColor = mix(fillColor, wireframeColor, I);
    }

    "#;

    let geometry_shader_src = r#"
    #version 330
    layout(triangles) in;
    layout(triangle_strip, max_vertices = 3) out;

    in Data
    {
        vec4 position;
    } vdata[3];

    out Data
    {
        noperspective out vec3 dist;
    } gdata;

    void main()
    {
        vec2 scale = vec2(500.0f, 500.0f); // scaling factor to make 'd' in frag shader big enough to show something
        vec2 p0 = scale * vdata[0].position.xy/vdata[0].position.w;
        vec2 p1 = scale * vdata[1].position.xy/vdata[1].position.w;
        vec2 p2 = scale * vdata[2].position.xy/vdata[2].position.w;

        vec2 v0 = p2-p1;
        vec2 v1 = p2-p0;
        vec2 v2 = p1-p0;
        float area = abs(v1.x*v2.y - v1.y*v2.x);

        gdata.dist = vec3(area/length(v0),0,0);
        gl_Position = vdata[0].position;
        EmitVertex();

        gdata.dist = vec3(0,area/length(v1),0);
        gl_Position = vdata[1].position;
        EmitVertex();

        gdata.dist = vec3(0,0,area/length(v2));
        gl_Position = vdata[2].position;
        EmitVertex();

        EndPrimitive();
    }
    "#;

    let program = glium::Program::from_source(&display,
            vertex_shader_src, fragment_shader_src,
            //None
            Some(geometry_shader_src)
            ).unwrap_or_else( |err| {
                println!("Error executing glium::Program::from_source: \n{}", err);
                std::process::exit(3)
            });

    fn to_uniform<'a, N: na::ToHomogeneous<Mat4<f32>>>(m: N) ->
        glium::uniforms::UniformsStorage<'a, [[f32; 4]; 4], glium::uniforms::EmptyUniforms> {
        uniform! { matrix: *na::to_homogeneous(&m).as_array() }
    }

    let build_uniform = |window_state: &WindowState, last_window_state: &WindowState| {
        let (x, y) = window_state.scaled_mouse_position;
        let (last_x, last_y) = window_state.last_scaled_mouse_position;

        let dx = x - last_x;
        let dy = y - last_y;
        let scale = 1.0;

        let last_rot = &last_window_state.uniform_mat.rotation;
        let rot =
            if window_state.is_left_drag {
                let vec = &Vec3::new(x/scale, y/scale, 0.0);

                Rot3::new(*vec)

                //let z = Vec3::new(0.0, 0.0, 1.0);

                //Rot3::new(na::rotate(last_rot, &Vec3::new(dx/scale, dy/scale, 0.0)))

                // let mut new_rot = last_rot.clone();
                // new_rot.look_at(vec, &z);
                // new_rot
            } else {
                *last_rot
            };

        let iso = Iso3::new_with_rotmat(
                    Vec3::new(x, y, 0.0), rot);

        iso
    };

    let mut last_window_state = WindowState {
        scaled_mouse_position: (1337.0, 1337.0),
        last_scaled_mouse_position: (1338.0, 1338.0),
        is_left_drag: false,
        uniform_mat: na::Iso3::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0))//na::Eye::new_identity(4)
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

        let uniform_mat = build_uniform(&window_state, &last_window_state);
        let uniforms = to_uniform(uniform_mat);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let params = glium::DrawParameters {
                        blending_function: Some(glium::BlendingFunction::Addition { source: SourceAlpha, destination: OneMinusSourceAlpha }),
                        ..
                        Default::default()
                    };


        target.draw(&vertex_buffer, &indices, &program,
                    &uniforms, &params).unwrap();
        target.finish();

        last_window_state = window_state;
        last_window_state.uniform_mat = uniform_mat;
    }
}

#[derive(Clone)]
struct WindowState {
    scaled_mouse_position: (f32, f32),
    last_scaled_mouse_position: (f32, f32),
    is_left_drag: bool,
    uniform_mat: Iso3<f32>
}


fn scale_mouse_position((wi, hi): (u32, u32), (xi, yi): (i32, i32)) -> (f32, f32) {
    let (w, h, x, y) = (wi as f32, hi as f32, xi as f32, yi as f32);
    (x/(w*2.0) - 1.0, -y/(h*2.0))
}
