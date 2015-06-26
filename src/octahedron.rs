use triangle_mesh::TriangleMesh;

pub fn octahedron(length: f32) -> TriangleMesh {
    let ratio = 0.1;
    let top_length = ratio * length;
    let thel: f32 = (2.0f32).sqrt() * top_length / 2.0;

    TriangleMesh {
        vertices: vec! [
            (  0.0,   0.0,        0.0),
            (-thel,  thel, top_length),
            ( thel,  thel, top_length),
            ( thel, -thel, top_length),
            (-thel, -thel, top_length),
            (  0.0,   0.0,     length)
        ],

        indices: vec! [
            [0, 1, 2],
            [0, 2, 3],
            [0, 3, 4],
            [0, 4, 1],

            [5, 2, 1],
            [5, 3, 2],
            [5, 4, 3],
            [5, 1, 4]
        ]
    }
}
