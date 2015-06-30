use std;
use std::fs::File;
use std::io::Read;
use core::ops::Mul;

use nalgebra as na;
use nalgebra::{Mat4, Iso3, Rot3, Vec3};

use glium::draw_parameters::LinearBlendingFactor::*;

use octahedron;
use icosphere;
use triangle_mesh::Mesh;
use scene_element::SceneElement;
use num::traits::Zero;

#[cfg(feature = "window")]
pub fn open_window() {
    use glium;
    use glium::{DisplayBuild, Surface};
    use glium::glutin::{Event, ElementState, MouseButton};

    fn read_glsl(filename: &str) -> String {
        let mut ret = String::new();

        File::open("glsl/".to_string() + filename + ".glsl")
            .and_then(|f| (&f).read_to_string(&mut ret))
                .unwrap();

        ret
    }

    let vertex_shader_src   = read_glsl("wireframe_vertex");
    let fragment_shader_src = read_glsl("wireframe_fragment");
    let geometry_shader_src = read_glsl("wireframe_geometry");

        let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    let program = glium::Program::from_source(&display,
            &vertex_shader_src, &fragment_shader_src,
            //None
            Some(&geometry_shader_src)
            ).unwrap_or_else( |err| {
                println!("Error executing glium::Program::from_source: \n{}", err);
                std::process::exit(3)
            });

    let build_uniform = |ws: &WindowState, last_ws: &WindowState, last_uniform: &Mat4<f32>| {
        let rot =
            if ws.is_left_drag {
                let arc_rot =
                    arcball_rotation(last_ws.scaled_mouse_position,
                                          ws.scaled_mouse_position);

                last_uniform.mul(na::to_homogeneous(&arc_rot))
            } else {
                *last_uniform
            };

        let (scroll_x, scroll_y) = ws.mouse_wheel_scroll;
        let translate = Vec3::new(scroll_x as f32, scroll_y as f32, 0.0);

        let trans_iso = Iso3::new_with_rotmat(
                    // Vec3::new(x, y, 0.0), Rot3::new(Vec3::new(0.0, 0.0, 0.0)));
                    translate, Rot3::new(Vec3::new(0.0, 0.0, 0.0)));

        na::to_homogeneous(&trans_iso) * rot
    };

    let mut last_window_state = WindowState {
        scaled_mouse_position: (1337.0, 1337.0),
        mouse_wheel_scroll: (0.0, 0.0),
        is_left_drag: false,
    };

    let mut last_uniform: Mat4<f32> = na::Eye::new_identity(4);

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

        let new_uniform = build_uniform(&window_state, &last_window_state, &last_uniform);

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
                    Vec3::new(0.0, 0.0, -0.0), Rot3::new(Vec3::new(0.0, 0.0, 0.0)));
        let mat2: Mat4<f32> = new_uniform * na::to_homogeneous(&trans_iso);
        let uniforms2 = uniform! { matrix: *mat2.as_array() };

        let transform = Iso3::new(Vec3::new(-0.5, 0.0, 0.0), Vec3::zero());
        let scene_elem =
            SceneElement {
                mesh: &icosphere::icosphere(0.1),
                transformations: vec![&transform]
            };

        let vertex_buffer = glium::VertexBuffer::new(&display, scene_elem.mesh.faces());
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        target.draw(&vertex_buffer, &indices, &program,
                    &uniforms2, &params).unwrap();

        target.finish();

        last_window_state = window_state;
        last_uniform = new_uniform;
    }
}

#[derive(Clone)]
struct WindowState {
    scaled_mouse_position: ScaledMousePosition,
    is_left_drag: bool,
    mouse_wheel_scroll: (f64, f64)
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

    Rot3::new(axis_in_camera_coord * angle * 4.0) // 1 / (360 / (2 * pi))
}
