#include "pcl.h"

#include <string.h>
#include <stdio.h>

#include "glad/glad.h"

void make_point_cloud(PointCloud* pcl, float col[3]) {
    unsigned int data_vbo;
    glGenBuffers(1, &data_vbo);

    unsigned int proj_vbo;
    glGenBuffers(1, &proj_vbo);

    unsigned int vao;
    glGenVertexArrays(1, &vao);

    glBindVertexArray(vao);
    glBindBuffer(GL_ARRAY_BUFFER, proj_vbo);
    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), 0);
    glEnableVertexAttribArray(0);

    glBindBuffer(GL_ARRAY_BUFFER, data_vbo);
    glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE, sizeof(float), 0);
    glEnableVertexAttribArray(1);
    
    glBindVertexArray(0);

    memcpy(pcl->col, col, sizeof(pcl->col));
    pcl->data = data_vbo;
    pcl->proj = proj_vbo;
    pcl->vao = vao;
}

void update_point_cloud_proj(PointCloud* pcl, const float* data, int count) {    
    glBindBuffer(GL_ARRAY_BUFFER, pcl->proj);
    glBufferData(GL_ARRAY_BUFFER, count * 3 * sizeof(float), data, GL_STATIC_DRAW);        
}

void update_point_cloud(PointCloud* pcl, const float* data, int count) {
    pcl->count = count;
    glBindBuffer(GL_ARRAY_BUFFER, pcl->data);
    glBufferData(GL_ARRAY_BUFFER, count * sizeof(float), data, GL_STATIC_DRAW);
}

void draw_point_cloud(PointCloud* pcl) {
    glBindVertexArray(pcl->vao);
    glDrawArrays(GL_POINTS, 0, pcl->count);
    glBindVertexArray(0);
}
