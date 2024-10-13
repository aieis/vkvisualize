#include <stdint.h>

struct PointCloud {
    unsigned int data;
    unsigned int proj;
    unsigned int vao;
    int count;
    float col[3];
};

typedef struct PointCloud PointCloud;
void make_point_cloud(PointCloud* pcl, float col[3]);
void update_point_cloud_proj(PointCloud* pcl, const float* data, int count);
void update_point_cloud(PointCloud* pcl, const float* data, int count);
void draw_point_cloud(PointCloud* pcl);
