#include "record.h"

#include <stdlib.h>
#include <stdio.h>

int write_whole_buffer(char* buf, int element_size, int count, FILE* fd) {
    int amount_written = 0;
    while (amount_written < count) {
        int am = fwrite(buf + amount_written * element_size, element_size, count - amount_written, fd);
        if (am == 0) {
            break;
        }
        amount_written += am;
    }
    return amount_written;
}

int read_whole_buffer(char* buf, int element_size, int count, FILE* fd) {

    int amount_read = 0;
    while (amount_read < count) {
        int am = fread(buf + amount_read * element_size, element_size, count - amount_read, fd);
        if (am == 0) {
            break;
        }
        amount_read += am;
    }

    return amount_read;
}


bool write_record_data(RecordData* record_data, const char* output) {
    FILE* fd = fopen(output, "wb");
    if (!fd) {
        return false;
    }

    fwrite(&record_data->width, sizeof(record_data->width), 1, fd);
    fwrite(&record_data->height, sizeof(record_data->height), 1, fd);
    fwrite(&record_data->frame_count, sizeof(record_data->frame_count), 1, fd);
    write_whole_buffer((char*)record_data->projection_data, 1, sizeof(float) * record_data->width * record_data->height * 3, fd);
    write_whole_buffer((char*)record_data->frame_data, sizeof(uint16_t), record_data->width * record_data->height * record_data->frame_count, fd);
    fclose(fd);
    return true;
}


bool read_record_data(RecordData* record_data, const char* filepath) {
    FILE* fd = fopen(filepath, "rb");
    if (!fd) {
        return false;
    }

    fread(&record_data->width, sizeof(record_data->width), 1, fd);
    fread(&record_data->height, sizeof(record_data->height), 1, fd);
    fread(&record_data->frame_count, sizeof(record_data->frame_count), 1, fd);

    record_data->projection_data = malloc(sizeof(float) * record_data->width * record_data->height * 3);
    read_whole_buffer((char*)record_data->projection_data, 1, sizeof(float) * record_data->width * record_data->height * 3, fd);

    record_data->frame_data = malloc(sizeof(uint16_t) * record_data->width * record_data->height * record_data->frame_count);
    read_whole_buffer((char*)record_data->frame_data, 1, sizeof(uint16_t) * record_data->width * record_data->height * record_data->frame_count, fd);
    fclose(fd);
    return true;
}
