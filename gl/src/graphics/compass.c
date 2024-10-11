#include "compass.h"
#include "glad/glad.h"

void make_compass(Compass* compass) {

    float vertices[] = {
        0.0, 0.0, 0.0,
        0.1, 0.0, 0.0,
        0.0, 0.1, 0.0,
        0.0, 0.0, 0.1
    };

    float colors[] = {
        1.0, 1.0, 1.0,
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0
    };

    unsigned int indices[] = {
        0, 1,
        0, 2,
        0, 3
    };

    unsigned int vbo[3] = {0};
    glCreateBuffers(3, vbo);
    
    unsigned int vao;
    glCreateVertexArrays(1, &vao);
    
    unsigned int vert = vbo[0];
    unsigned int col= vbo[1];
    unsigned int ind = vbo[2];

    glBindBuffer(GL_ARRAY_BUFFER, vert);
    glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);

    glBindBuffer(GL_ARRAY_BUFFER, col);
    glBufferData(GL_ARRAY_BUFFER, sizeof(colors), colors, GL_STATIC_DRAW);

    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ind);
    glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(indices), indices, GL_STATIC_DRAW);
    
    glBindVertexArray(vao);

    glBindBuffer(GL_ARRAY_BUFFER, vert);
    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), 0);
    glEnableVertexAttribArray(0);

    glBindBuffer(GL_ARRAY_BUFFER, col);
    glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), 0);
    glEnableVertexAttribArray(1);

    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ind);

    glBindVertexArray(0);

    compass->vert = vert;
    compass->col = col;
    compass->ind = ind;
    compass->vao = vao;
}

void draw_compass(Compass* compass, int color_loc) {
    glBindVertexArray(compass->vao);
    glDrawElements(GL_LINES, 6, GL_UNSIGNED_INT, 0);
    glBindVertexArray(0);
}



