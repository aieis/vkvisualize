struct PointCloud {
    unsigned int data;
    unsigned int vao;
    int count;
    float col[3];
};

typedef struct PointCloud PointCloud;
void make_point_cloud(PointCloud* pcl, float col[3]);
void update_point_cloud(PointCloud* pcl, void* data, int size);
void draw_point_cloud(PointCloud* pcl);
