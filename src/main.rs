extern crate assimp;

use std::env;
mod glium_mod;
#[macro_use]
extern crate glium;

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
  glium_mod::open_window();
}
