#scope_module

Quad_Mesh :: struct {
    vertices: [16]float : .[
         1,  1,     1, 1,
         1, -1,     1, 0,
        -1, -1,     0, 0,
        -1,  1,     0, 1,
    ];
    indices:  [6]u32 : .[
        0, 1, 3,
        1, 2, 3,
    ];
    vao:      u32;
    vbo:      u32;
    ebo:      u32;
}

create_mesh :: () -> Quad_Mesh {
    mesh: Quad_Mesh;

    glGenVertexArrays(1, *mesh.vao);
    glGenBuffers(1, *mesh.vbo);
    glGenBuffers(1, *mesh.ebo);

    glBindVertexArray(mesh.vao);

    glBindBuffer(GL_ARRAY_BUFFER, mesh.vbo);
    glBufferData(
        GL_ARRAY_BUFFER,
        size_of(type_of(mesh.vertices.data)) * mesh.vertices.count,
        mesh.vertices.data,
        GL_STATIC_DRAW,
    );

    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, mesh.ebo);
    glBufferData(
        GL_ELEMENT_ARRAY_BUFFER,
        size_of(type_of(mesh.indices.data)) * mesh.indices.count,
        mesh.indices.data,
        GL_STATIC_DRAW,
    );

    // Position
    glVertexAttribPointer(
        0,
        2,
        GL_FLOAT,
        GL_FALSE,
        4 * size_of(float),
        cast(*void) 0,
    );
    glEnableVertexAttribArray(0);

    // UV
    glVertexAttribPointer(
        1,
        2,
        GL_FLOAT,
        GL_FALSE,
        4 * size_of(float),
        cast(*void) (2 * size_of(float)),
    );
    glEnableVertexAttribArray(1);

    glBindVertexArray(0); 
    glBindBuffer(GL_ARRAY_BUFFER, 0); 
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);

    return mesh;
}

destroy_mesh :: (using mesh: Quad_Mesh) {
    glDeleteVertexArrays(1, *vao);
    glDeleteBuffers(1, *vbo);
    glDeleteBuffers(1, *ebo);
}
