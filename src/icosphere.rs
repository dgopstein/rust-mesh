use triangle_mesh::TriangleMesh;

pub fn icosphere(r: f32) -> TriangleMesh {
    let t = r * (1.0 + 5f32.sqrt()) / 2.0;

    TriangleMesh {
        vertices: vec! [
            (-r,  t,  0.0),
            ( r,  t,  0.0),
            (-r, -t,  0.0),
            ( r, -t,  0.0),

            ( 0.0, -r,  t),
            ( 0.0,  r,  t),
            ( 0.0, -r, -t),
            ( 0.0,  r, -t),

            ( t,  0.0, -r),
            ( t,  0.0,  r),
            (-t,  0.0, -r),
            (-t,  0.0,  r)
        ],

        indices: vec! [
            // 5 faces around point 0
            [0, 11,  5],
            [0,  5,  1],
            [0,  1,  7],
            [0,  7, 10],
            [0, 10, 11],

            // 5 adjacent faces
            [ 1,  5, 9],
            [ 5, 11, 4],
            [11, 10, 2],
            [10,  7, 6],
            [ 7,  1, 8],

            // 5 faces around point 3
            [3, 9, 4],
            [3, 4, 2],
            [3, 2, 6],
            [3, 6, 8],
            [3, 8, 9],

            // 5 adjacent faces
            [4, 9,  5],
            [2, 4, 11],
            [6, 2, 10],
            [8, 6,  7],
            [9, 8,  1]
        ]
    }
}
