#include "pcl_proc.h"

#include <stdlib.h>
#include <string.h>

void make_pcl_processor(PclProcessor* pcl, int width, int height, int frame_count) {
    pcl->width = width;
    pcl->height = height;
    pcl->frame_count = frame_count;
    pcl->depths = malloc(sizeof(float) * width * height);
    pcl->proximity = malloc(sizeof(float) * width * height * 2);
    memset(pcl->proximity, 0, sizeof(float) * width * height * 2);
    
    pcl->idx = 0;
}

void enqueue_pcl_frame(PclProcessor* pcl, float* data) {
    memset(pcl->depths, 0, sizeof(float) * pcl->width * pcl->height);
    for (int row = 1; row < pcl->height - 1; row++) {
        for (int col = 1; col < pcl->width - 1; col++) {
            int offset = row * pcl->width + col;
            float p = data[offset];
            float r = data[offset + 1];
            float b = data[offset + pcl->width];

            pcl->proximity[offset * 2 + 1] = p - r;
            pcl->proximity[(offset + pcl->width)*2] = p - b;

            float px[] = {
                pcl->proximity[offset * 2 - 1],
                pcl->proximity[(offset - pcl->width)*2],
                pcl->proximity[offset * 2 + 1],
                pcl->proximity[(offset + pcl->width)*2],
            };
            
            float avg = (px[0] + px[1] + px[2] + px[3]) / 4;
            avg = avg >= 0? avg : -avg;
            
            pcl->depths[offset] = avg > 100? p: 0;
        }
    }
}
