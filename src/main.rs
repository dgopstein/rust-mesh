extern crate assimp;

use std::env;

fn read_scene_from_args() {
  let print_usage_and_die = || {
      println!("Usage: rust-mesh filename");
      std::process::exit(1)
    };

  let filename = env::args().nth(1).unwrap_or_else(print_usage_and_die).to_owned();

  println!("Loading file: {}", filename);

  let ref ai_scene  = *assimp::load(&filename, 0).unwrap_or_else(|str| {
    println!("An error occured parsing the scene: {}", str);
    std::process::exit(2)
  });

  println!("aiScene.mNumMeshes: {}", ai_scene.mNumMeshes);
}

fn main() {
  let ai_scene = read_scene_from_args();
  glfw_main(||{octahedron(1)});
}


fn octahedron(length: f32) {
    let ratio = 0.1;
    let top_length = ratio * length;
    let thel: f32 = (2.0f32).sqrt() * top_length / 2.0;

    let verticies = [
        [  0.0,   0.0,        0.0],
        [-thel,  thel, top_length],
        [ thel,  thel, top_length],
        [ thel, -thel, top_length],
        [-thel, -thel, top_length],
        [  0.0,   0.0,     length]];


    gl::Disable(GL_LIGHTING);
    gl::PolygonMode(GL_FRONT_AND_BACK, GL_LINE);

    gl::Begin(GL_TRIANGLES);
    gl::Vertex3dv(p0.data()); gl::Vertex3dv(p1.data()); gl::Vertex3dv(p2.data());
    gl::Vertex3dv(p0.data()); gl::Vertex3dv(p2.data()); gl::Vertex3dv(p3.data());
    gl::Vertex3dv(p0.data()); gl::Vertex3dv(p3.data()); gl::Vertex3dv(p4.data());
    gl::Vertex3dv(p0.data()); gl::Vertex3dv(p4.data()); gl::Vertex3dv(p1.data());

    gl::Vertex3dv(p5.data()); gl::Vertex3dv(p2.data()); gl::Vertex3dv(p1.data());
    gl::Vertex3dv(p5.data()); gl::Vertex3dv(p3.data()); gl::Vertex3dv(p2.data());
    gl::Vertex3dv(p5.data()); gl::Vertex3dv(p4.data()); gl::Vertex3dv(p3.data());
    gl::Vertex3dv(p5.data()); gl::Vertex3dv(p1.data()); gl::Vertex3dv(p4.data());
    gl::End();

    gl::Enable(GL_LIGHTING);
}

extern crate gl;
extern crate glfw;

use glfw::{Action, Context, Key};
use gl::types::*;

fn glfw_main<F, A>(do_stuff: F)
        where F : FnOnce() -> A {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s));

    do_stuff();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
