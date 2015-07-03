use assimp::ffi::*;

pub struct Bone<'a> {
    parent: Option<&'a Bone<'a>>,
    childen: Vec<&'a Bone<'a>>
}

pub fn map_nodes<R>(f: &Fn(&aiNode) -> R, n: &aiNode) -> Vec<R>
    // where F : Fn(&aiNode) -> R {
    {

    let mut v: Vec<R> = Vec::new();
    let mapped: R = f(n);
    v.push(mapped);

    let children = n.children();

    let mapped_children =
        children.iter().flat_map( |child|
            map_nodes(f, &child)
        );

    v.extend( mapped_children );

    v
}

// use core;
// pub fn map_nodes<F, R>(f: F, n: Box<aiNode>) -> Vec<R>
//     where F : FnMut(Box<aiNode>) -> R {
//
//     let mut all_nodes = vec![n];
//     {
//         let mut borrowed = &mut all_nodes;
//
//         for &mut node in borrowed {
//             borrowed.extend(node.children());
//         }
//     }
//
//     let all_mapped: Vec<R> = all_nodes.map_in_place(f);
//
//     all_mapped
// }

pub fn parse_scene(scene: &aiScene) -> Option<Bone> {
    println!("Root node name: {}", scene.root_node().name());
    println!("All names: {:?}", map_nodes(&|x|x.name(), &scene.root_node()));

    None
}
