use triangle_mesh as mesh;
use nalgebra as na;
use std::rc::Rc;

// trait Transformation: na::ToHomogeneous<na::Mat4<f32>> + Clone {}
// impl<T: na::ToHomogeneous<na::Mat4<f32> + Clone> Transformation for T {}

pub type Transformation = na::ToHomogeneous<na::Mat4<f32>>;
pub type Transformations = [Rc<na::ToHomogeneous<na::Mat4<f32>>>];

trait Transformable<'a> {
    fn transformations(&self) -> &Vec<Rc<na::ToHomogeneous<na::Mat4<f32>> + 'a>>;

    fn transformation(&self) -> na::Mat4<f32> {
        self.transformations().iter()
            .fold(na::Eye::new_identity(4), |sum, trans| {
                 sum * trans.to_homogeneous() })
    }
}

pub struct SceneElement<'a> {
    pub name: String,
    pub mesh: Rc<mesh::Mesh>,
    pub transformations: Box<Vec<Rc<na::ToHomogeneous<na::Mat4<f32>> + 'a>>>,
}

impl<'a> Transformable<'a> for SceneElement<'a> {
    fn transformations(&self) -> &Vec<Rc<na::ToHomogeneous<na::Mat4<f32>> + 'a>> { &*self.transformations }
}
