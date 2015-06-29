pub type P3 = (f32, f32, f32);
pub type TriangleIndices = [usize; 3];

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 4],
}

implement_vertex!(Vertex, position);

pub struct TriangleMesh {
    pub vertices: Vec<P3>,
    pub indices: Vec<TriangleIndices>
}

pub trait Mesh {
    fn faces(&self) -> Vec<Vertex>;
}

impl Mesh for TriangleMesh {
    fn faces(&self) -> Vec<Vertex> {
        self.indices.iter().flat_map( |idxs| {
            idxs.iter().flat_map( |idx| {
                self.vertices.get(*idx).map( |v|
                    Vertex{position: [v.0, v.1, v.2, 1.0]} )
        })}).collect::<Vec<_>>()
    }
}
