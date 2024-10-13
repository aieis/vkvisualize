#include <stdbool.h>
#include <stdint.h>

struct RecordData {
    int width;
    int height;
    int frame_count;
    float* projection_data;
    uint16_t* frame_data;
};

typedef struct RecordData RecordData;
bool write_record_data(RecordData* record_data, const char* output);
bool read_record_data(RecordData* record_data, const char* filepath);
