use assimp::ffi::*;
use nalgebra as na;
use scene_element::{SceneElement, Transformation, Transformations};
use octahedron::octahedron;
use core::clone;
use triangle_mesh as mesh;
use std::rc::Rc;

pub struct Bone<'a> {
    parent: Option<&'a Bone<'a>>,
    childen: Vec<&'a Bone<'a>>,
    element: SceneElement<'a>
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
   v.iter().map(|x| x.clone()).collect()
}


pub fn build_scene_elements<'a>(node: &aiNode) -> Vec<SceneElement<'a>> {
    fn bse<'a>(n: &aiNode, transforms: &Vec<Rc<na::ToHomogeneous<na::Mat4<f32>> + 'a>>) -> Vec<SceneElement<'a>> {
        let elem: SceneElement<'a> = SceneElement {
            name: n.name(),
            mesh: Rc::new(octahedron(0.2)),
            transformations: Box::new(shallow_copy(transforms))
        };

        let new_transforms =
            using(shallow_copy(transforms), |t| {
                t.push(Rc::new(Homo4(a2d_to_na(n.transformation()))))
            });

        using(vec![elem], |elems| {
            for child in n.children().iter() {
                elems.extend(bse(&child, &new_transforms));
            }
        })
    }

    bse(node, &Vec::new())
}

fn format_mat(x: &na::ToHomogeneous<na::Mat4<f32>>) -> String {
    format!("{:?}", x.to_homogeneous())
}

pub fn parse_scene(scene: &aiScene) -> Option<Bone> {
    println!("Root node name: {}", scene.root_node().name());
    println!("All node names: {:?}", map_nodes(|x|x.name(), &scene.root_node()));

    let elems = build_scene_elements(&scene.root_node());
    println!("All scene elems: {:?}", elems.iter().map(|x|x.name.to_string()).collect::<Vec<_>>());

    for elem in elems {
        println!("name: {}", elem.name);

        for (i, trans) in elem.transformations.iter().enumerate() {
            println!("{}: {}", i, format_mat(&**trans));
        }

        println!("");
    }

    None
}
