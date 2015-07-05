use assimp::ffi::*;
use nalgebra as na;
use scene_element::{SceneElement, Transformation, Transformations};
use octahedron::octahedron;
use core::clone;
use triangle_mesh as mesh;
use std::rc::Rc;

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

// fn ai2na(m: aiMatrix4x4) -> na::Mat4<f32> {
//     Mat4::new(m.a1, m.a2, m.a3, m.a4,
//               m.b1, m.b2, m.b3, m.b4,
//               m.c1, m.c2, m.c3, m.c4,
//               m.d1, m.d2, m.d3, m.d4)
// }

fn a2d_to_na(m: [[f32; 4]; 4]) -> na::Mat4<f32> {
    na::Mat4::new(m[0][0], m[0][1], m[0][2], m[0][3],
              m[1][0], m[1][1], m[1][2], m[1][3],
              m[2][0], m[2][1], m[2][2], m[2][3],
              m[3][0], m[3][1], m[3][2], m[3][3])
}

#[derive(Copy, Clone)]
pub struct Homo4(na::Mat4<f32>);

impl na::ToHomogeneous<na::Mat4<f32>> for Homo4 {
    fn to_homogeneous(&self) -> na::Mat4<f32> {
        self.0.clone()
    }
}


fn shallow_copy<'a>(v: &Vec<Rc<na::ToHomogeneous<na::Mat4<f32>> + 'a>>) -> Vec<Rc<na::ToHomogeneous<na::Mat4<f32>> + 'a>> {
//fn shallow_copy<'a, 'b>(v: &'a [Rc<na::ToHomogeneous<na::Mat4<f32>>>]) -> Vec<Rc<na::ToHomogeneous<na::Mat4<f32>> + 'b>> {
   v.iter().map(|x| x.clone()).collect()
}


pub fn build_scene_elements<'a>(node: &aiNode) -> Vec<SceneElement<'a>> {
    // fn bse<'a>(n: &aiNode, transforms: Vec<&Homo4<f32>>) -> Vec<SceneElement<'a>>{
    // fn bse<'a>(n: &aiNode, transforms: Vec<&na::ToHomogeneous<na::Mat4<f32>>>) -> Vec<SceneElement<'a>>{
    fn bse<'a>(n: &aiNode, transforms: &Vec<Rc<na::ToHomogeneous<na::Mat4<f32>> + 'a>>) -> Vec<SceneElement<'a>> {
        // let transforms_copy: Vec<Rc<na::ToHomogeneous<na::Mat4<f32>> + 'a>>
            // = shallow_copy(&transforms);
    //    let transforms_copy: Box<Vec<Rc<na::ToHomogeneous<na::Mat4<f32>> + 'a>>>
                // = transforms.iter().map(|x| x.clone()).collect();
                // = Box::new(shallow_copy(&transforms));

        let elem: SceneElement<'a> = SceneElement {
            name: n.name(),
            mesh: Rc::new(octahedron(0.2)),
            transformations: Box::new(shallow_copy(transforms))
        };

        let new_transforms =
            using(shallow_copy(transforms), |t| {
                // let trans1: Transformation = Homo4(a2d_to_na(n.transformation()));
                // let trans: &Box<Transformation> = &Box::new(trans1);
                // let v: Transformations<'a> = vec![trans];
                t.push(Rc::new(Homo4(a2d_to_na(n.transformation()))))
            });

        // using(vec![elem], |v| {
        //     v.extend(n.children().iter().flat_map( |child|
        //         bse(&child, new_transforms)));
        // })



        let mut elems = vec![elem];

        for child in n.children().iter() {
            let child_elems = bse(&child, &new_transforms);
            elems.extend(child_elems);
        }

        elems
    }

    bse(node, &Vec::new())
}

pub fn parse_scene(scene: &aiScene) -> Option<Bone> {
    println!("Root node name: {}", scene.root_node().name());
    println!("All node names: {:?}", map_nodes(|x|x.name(), &scene.root_node()));
    println!("All scene elems: {:?}", build_scene_elements(&scene.root_node()).iter().map(|x|x.name.to_string()).collect::<Vec<_>>());

    None
}
