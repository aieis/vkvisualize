#include "record.h"

#include <stdlib.h>
#include <stdio.h>

bool write_record_data(RecordData* record_data, const char* output) {
    FILE* fd = fopen(output, "w");
    if (!fd) {
        return false;
    }

    fwrite(&record_data->width, sizeof(record_data->width), 1, fd);
    fwrite(&record_data->height, sizeof(record_data->height), 1, fd);
    fwrite(&record_data->frame_count, sizeof(record_data->frame_count), 1, fd);
    fwrite(record_data->projection_data, sizeof(float), record_data->width * record_data->height * 3, fd);
    fwrite(record_data->frame_data, sizeof(uint16_t), record_data->width * record_data->height * record_data->frame_count, fd);
    return true;
}


bool read_record_data(RecordData* record_data, const char* filepath) {
    FILE* fd = fopen(filepath, "r");
    if (!fd) {
        return false;
    }

    fread(&record_data->width, sizeof(record_data->width), 1, fd);
    fread(&record_data->height, sizeof(record_data->height), 1, fd);
    fread(&record_data->frame_count, sizeof(record_data->frame_count), 1, fd);

    record_data->projection_data = malloc(sizeof(float) * record_data->width * record_data->height * 3);
    fread(record_data->projection_data, sizeof(float), record_data->width * record_data->height * 3, fd);

    record_data->frame_data = malloc(sizeof(uint16_t) * record_data->width * record_data->height * record_data->frame_count);
    fread(record_data->frame_data, sizeof(uint16_t), record_data->width * record_data->height * record_data->frame_count, fd);
    return true;
}
