use std;
use std::fs::File;
use std::io::Read;
use core::ops::Mul;

use nalgebra as na;
use nalgebra::{Mat4, Iso3, Rot3, Vec3};

use glium::draw_parameters::LinearBlendingFactor::*;

use octahedron;
use icosphere;
use triangle_mesh::{Mesh, Vertex};
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

    let mut counter = 0;

    for event in display.wait_events() {
        counter += 1;
        println!("counter: {}", counter);

        let mut target = display.draw();
        target.clear_color(0.1, 0.3, 0.6, 1.0);

        let params = glium::DrawParameters {
                        blending_function: Some(glium::BlendingFunction::Addition { source: SourceAlpha, destination: OneMinusSourceAlpha }),
                        ..
                        Default::default()
                    };

        let ident4: Mat4<f32> = na::Eye::new_identity(4);
        let uniforms2 = uniform! { matrix: *ident4.as_array() };

        // Changing the vertex buffer changed the counter from 227 to 282
        let vec_vert: Vec<Vertex> = Vec::new();
        let vertex_buffer = glium::VertexBuffer::new(&display, vec_vert);
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let draw_res = target.draw(&vertex_buffer, &indices, &program,
                    &uniforms2, &params);

        match draw_res {
            Ok(unit) => { println!("Ok({:?})", unit) }
            Err(err) => { println!("Err({:?})", err) }
        }

        target.finish();
    }
}
