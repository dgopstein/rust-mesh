extern crate assimp;

use std::env;

fn main() {
  let print_usage_and_die = || {
      println!("Usage: rust-mesh filename");
      std::process::exit(1)
    };

  let filename = &env::args().nth(1).unwrap_or_else(print_usage_and_die);

  println!("Loading file: {}", filename);

  let aiScene  = assimp::load(filename, 0).unwrap_or_else(|str| {
    println!("An error occured parsing the scene: {}", str);
    std::process::exit(2)
  });

  println!("aiScene.mNumMeshes: {}", aiScene.mNumMeshes);
}
