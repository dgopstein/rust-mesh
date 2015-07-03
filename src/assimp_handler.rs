use assimp::ffi::*;

pub struct Bone<'a> {
    parent: Option<&'a Bone<'a>>,
    childen: Vec<&'a Bone<'a>>
}

fn using<X, F, R>(mut x: X, f: F) -> X
        where F: Fn(&mut X) -> R {
    f(&mut x);
    x
}

pub fn map_nodes<F, R>(f: F, n: &aiNode) -> Vec<R>
        where F : Fn(&aiNode) -> R {
    map_nodes_rec(&f, n)
}

fn map_nodes_rec<R>(f: &Fn(&aiNode) -> R, n: &aiNode) -> Vec<R> {
    using(vec![f(n)], |v| {
        v.extend(n.children().iter().flat_map(|child| map_nodes(f, &child)));
    })
}

pub fn parse_scene(scene: &aiScene) -> Option<Bone> {
    println!("Root node name: {}", scene.root_node().name());
    println!("All node names: {:?}", map_nodes(|x|x.name(), &scene.root_node()));

    None
}
