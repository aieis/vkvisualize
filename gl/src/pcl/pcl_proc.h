struct PclProcessor {
    int width;
    int height;
    int frame_count;

    float* depths;
    float* proximity;
    int idx;
};

typedef struct PclProcessor PclProcessor;


void make_pcl_processor(PclProcessor* pcl, int width, int height, int frame_count);
void enqueue_pcl_frame(PclProcessor* pcl, float* data);
