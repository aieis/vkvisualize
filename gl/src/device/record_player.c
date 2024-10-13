#include "record_player.h"

#include <stdio.h>

bool make_record_player(RecordPlayer* record_player, const char* file) {

    if (!read_record_data(record_player->record_data, file)) {
        return false;
    }

    record_player->width = record_player->record_data->width;
    record_player->height = record_player->record_data->height;
    record_player->count = record_player->width * record_player->height;
    
    record_player->idx = 0;
    record_player->projection_data = record_player->record_data->projection_data;

    float* p = record_player->projection_data;
    printf("Point: (%f, %f, %f)\n",  p[0], p[1], p[2]);

    p = record_player->projection_data + 639 * 3;
    printf("Point: (%f, %f, %f)\n",  p[0], p[1], p[2]);

    p = record_player->projection_data + 640 * 479 * 3;
    printf("Point: (%f, %f, %f)\n",  p[0], p[1], p[2]);
    
    p = record_player->projection_data + (640 * 480 - 1) * 3;
    printf("Point: (%f, %f, %f)\n",  p[0], p[1], p[2]);
    return true;
}


bool poll_record_player(RecordPlayer* record_player, float** projection_data, uint16_t** data) {
    *projection_data = record_player->projection_data;
    *data = record_player->record_data->frame_data + record_player->idx * record_player->count;
    record_player->idx = (record_player->idx + 1) % record_player->record_data->frame_count;
    return true;
}

