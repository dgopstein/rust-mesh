use assimp::ffi::aiScene;

pub struct Bone<'a> {
    parent: Option<&'a Bone<'a>>,
    childen: Vec<&'a Bone<'a>>
}

// pub fn map_nodes<B>(f: aiNode -> B) -> Vec<B> {
//     aiNode.mChildren
// }

pub fn parse_scene(scene: &aiScene) -> Option<Bone> {
    println!("Root node name: {}", unsafe{(*scene.mRootNode).name()});
    println!("");

    None
}
