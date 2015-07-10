#![feature(core)]
#![feature(unboxed_closures)]

extern crate assimp;

use std::env;

#[macro_use]
extern crate glium;
extern crate glutin;
extern crate nalgebra;
extern crate core;
extern crate num;

mod glium_mod;
mod scene_element;
mod triangle_mesh;
mod octahedron;
mod icosphere;
mod assimp_handler;

fn read_scene_from_args<'a>() -> Box<assimp::ffi::aiScene> {
  let print_usage_and_die = || {
      println!("Usage: rust-mesh filename");
      std::process::exit(1)
    };

  let filename = env::args().nth(1).unwrap_or_else(print_usage_and_die).to_owned();

  println!("Loading file: {}", filename);

  assimp::load(&filename, 0).unwrap_or_else(|str| {
    println!("An error occured parsing the scene: {}", str);
    std::process::exit(2)
  })
}

fn main() {
  let ai_scene = read_scene_from_args();
  println!("aiScene.mNumMeshes: {}", ai_scene.mNumMeshes);

  let x = (*ai_scene.root_node()); //XXX This line causes the crash. Comment it out to see it not fail.

  // assimp_handler::parse_scene(&ai_scene);

  glium_mod::open_window();
}
