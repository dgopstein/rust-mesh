use std;
use core::ops::Mul;

use nalgebra as na;
use nalgebra::{Mat4, Iso3, Rot3, Vec3};

use glium::draw_parameters::LinearBlendingFactor::*;

use octahedron;
use icosphere;
use triangle_mesh::Mesh;

#[cfg(feature = "window")]
pub fn open_window() {
    use glium;
    use glium::{DisplayBuild, Surface};
    use glium::glutin::{Event, ElementState, MouseButton};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let shape = icosphere::icosphere(0.1).faces();//octahedron::octahedron(0.5).faces();

    let vertex_buffer = glium::VertexBuffer::new(&display, shape);
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

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
        let last_mouse = window_state.last_scaled_mouse_position;
        let cur_mouse = window_state.scaled_mouse_position;
        let (x, y) = cur_mouse;


        // let last_rot = &last_window_state.uniform_mat.rotation;
        let last_rot = &last_window_state.uniform_mat;
        let rot =
            if window_state.is_left_drag {
                let arc_rot = arcball_rotation(last_mouse, cur_mouse);

                // last_rot.mul(arc_rot)
                last_rot.mul(na::to_homogeneous(&arc_rot))
            } else {
                *last_rot
            };

        let (scroll_x, scroll_y) = window_state.mouse_wheel_scroll;
        let translate = Vec3::new(scroll_x as f32, scroll_y as f32, 0.0);

        let trans_iso = Iso3::new_with_rotmat(
                    // Vec3::new(x, y, 0.0), rot);
                    // Vec3::new(0.0, 0.0, 0.0), Rot3::new(Vec3::new(0.0, 0.0, 0.0)));
                    //Vec3::new(x, y, 0.0), Rot3::new(Vec3::new(0.0, 0.0, 0.0)));
                    // Vec3::new(0.0, 0.0, 0.0), Rot3::new(Vec3::new(0.0, 0.0, 0.0)));
                    translate, Rot3::new(Vec3::new(0.0, 0.0, 0.0)));

        // let rot_iso = Iso3::new_with_rotmat(
        //         Vec3::new(0.0, 0.0, 0.0), rot);

        na::to_homogeneous(&trans_iso) * rot
    };

    let mut last_window_state = WindowState {
        scaled_mouse_position: (1337.0, 1337.0),
        last_scaled_mouse_position: (1338.0, 1338.0),
        mouse_wheel_scroll: (0.0, 0.0),
        is_left_drag: false,
        uniform_mat: na::Eye::new_identity(4)
    };
    for event in display.wait_events() {
        let mut window_state = last_window_state.clone();

        let (size, scale) =
            display.get_window().map(|win| {

                (win.get_inner_size().unwrap_or((1337, 1337)),
                 win.hidpi_factor())
            }).unwrap_or(((1337, 1337), 1.0));

        match event {
            Event::Closed => {
                println!("Closing: {:?}", event);
                break;
            }
            Event::MouseMoved((x, y)) => {
                // println!("Mouse: ({}, {})", x, y);
                // println!("window ize: {:?}", size);
                let scaled_x = ((x as f32) / scale).round() as i32;
                let scaled_y = ((y as f32) / scale).round() as i32;

                let scaled_mouse = scale_mouse_position(size, (scaled_x, scaled_y));
                // println!("scaled_mouse: {:?}", scaled_mouse);
                window_state.scaled_mouse_position = scaled_mouse;
            }
            Event::MouseWheel(x, y) => {
                window_state.mouse_wheel_scroll = (x / size.0 as f64, -y / size.1 as f64);
                },
            Event::MouseInput(action, button) => {
                match (action, button) {
                    (ElementState::Pressed,  MouseButton::Left) => { window_state.is_left_drag = true; }
                    (ElementState::Released, MouseButton::Left) => { window_state.is_left_drag = false; }
                    _ => {}
                }
            }
            // event => println!("Event: {:?}", event)
            _ => {}
        }

        let uniform_mat = build_uniform(&window_state, &last_window_state);
        // let uniforms = to_uniform(uniform_mat);
        let uniforms = uniform! { matrix: *uniform_mat.as_array() };


        let mut target = display.draw();
        target.clear_color(0.1, 0.3, 0.6, 1.0);

        let params = glium::DrawParameters {
                        blending_function: Some(glium::BlendingFunction::Addition { source: SourceAlpha, destination: OneMinusSourceAlpha }),
                        ..
                        Default::default()
                    };

        // target.draw(&vertex_buffer, &indices, &program,
                    // &uniforms, &params).unwrap();

        let trans_iso = Iso3::new_with_rotmat(
                    Vec3::new(0.0, 0.0, -0.15), Rot3::new(Vec3::new(0.0, 0.0, 0.0)));
        let mat2: Mat4<f32> = uniform_mat * na::to_homogeneous(&trans_iso);
        let uniforms2 = uniform! { matrix: *mat2.as_array() };

        target.draw(&vertex_buffer, &indices, &program,
                    &uniforms2, &params).unwrap();

        target.finish();

        last_window_state = window_state;
        last_window_state.uniform_mat = uniform_mat;
    }
}

#[derive(Clone)]
struct WindowState {
    scaled_mouse_position: ScaledMousePosition,
    last_scaled_mouse_position: ScaledMousePosition,
    is_left_drag: bool,
    mouse_wheel_scroll: (f64, f64),
    uniform_mat: Mat4<f32>
}

fn scale_mouse_position((wi, hi): (u32, u32), (xi, yi): (i32, i32)) -> ScaledMousePosition {
    let (w, h, x, y) = (wi as f32, hi as f32, xi as f32, yi as f32);
    (2.0*x/w - 1.0, 1.0-2.0*y/h)
}

type ScaledMousePosition = (f32, f32);

// https://en.wikibooks.org/wiki/OpenGL_Programming/Modern_OpenGL_Tutorial_Arcball
fn arcball_vector((x, y): ScaledMousePosition) -> Vec3<f32> {
    let op_squared = x * x + y * y;

    if op_squared <= 1.0 {
        let z = (1.0 - op_squared).sqrt();  // Pythagorean
        Vec3::new(x, y, z)
    } else {
        na::normalize(&Vec3::new(x, y, 0.0))  // nearest point
    }
}

fn arcball_rotation(last: ScaledMousePosition, cur: ScaledMousePosition) -> Rot3<f32> {
    let va = arcball_vector(last);
    let vb = arcball_vector(cur);
    let angle = na::dot(&va, &vb).min(1.0).acos();
    let axis_in_camera_coord = na::cross(&va, &vb);
    // glm::mat3 camera2object = glm::inverse(glm::mat3(transforms[MODE_CAMERA]) * glm::mat3(mesh.object2world));
    // glm::vec3 axis_in_object_coord = camera2object * axis_in_camera_coord;
    // mesh.object2world = glm::rotate(mesh.object2world, glm::degrees(angle), axis_in_object_coord);

    Rot3::new(axis_in_camera_coord * angle * 0.07) // 1 / (360 / (2 * pi))
}
