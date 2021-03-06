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

fn read_scene_from_args<'a>() -> &'a assimp::ffi::aiScene {
  let print_usage_and_die = || {
      println!("Usage: rust-mesh filename");
      std::process::exit(1)
    };

  let filename = env::args().nth(1).unwrap_or_else(print_usage_and_die).to_owned();

  println!("Loading file: {}", filename);

  let mut _ai_scene: &'a assimp::ffi::aiScene = assimp::load(&filename, 0).unwrap_or_else(|str| {
    println!("An error occured parsing the scene: {}", str);
    std::process::exit(2)
  });

  let ai_scene: &'a assimp::ffi::aiScene = &_ai_scene;

  ai_scene
}

fn main() {
  let ai_scene = read_scene_from_args();
  println!("aiScene.mNumMeshes: {}", ai_scene.mNumMeshes);
  assimp_handler::parse_scene(&ai_scene);

  glium_mod::open_window();
}
