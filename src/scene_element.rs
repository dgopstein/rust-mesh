use triangle_mesh as mesh;
use nalgebra as na;
use nalgebra::{Mat3, Mat4};

pub type Transformation = na::ToHomogeneous<Mat4<f32>>;
pub type Transformations<'a> = Vec<&'a Transformation>;

trait Transformable<'a> {
    fn transformations(&self) -> &Transformations<'a>;

    fn transformation(&self) -> Mat4<f32> {
        self.transformations().iter()
            .fold(na::Eye::new_identity(4), |sum, trans| {
                 sum * trans.to_homogeneous() })
    }
}

pub struct SceneElement<'a> {
    pub mesh: &'a mesh::Mesh,
    pub transformations: Transformations<'a>
}

impl<'a> Transformable<'a> for SceneElement<'a> {
    fn transformations(&self) -> &Transformations<'a> { &self.transformations }
}