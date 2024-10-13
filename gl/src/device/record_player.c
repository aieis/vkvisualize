#include "record_player.h"
#include <stdlib.h>
#include <stdio.h>

bool make_record_player(RecordPlayer* record_player, const char* file) {

    if (!read_record_data(&record_player->record_data, file)) {
        return false;
    }

    printf("Record: '%s':\n", file);
    printf("\tDims: %dx%dx%d\n"
           , record_player->record_data.width
           , record_player->record_data.height
           , record_player->record_data.frame_count);
    
    record_player->width = record_player->record_data.width;
    record_player->height = record_player->record_data.height;
    record_player->count = record_player->width * record_player->height;
    record_player->frame = malloc(record_player->count * sizeof(float));
    record_player->idx = 0;
    return true;
}


bool poll_record_player(RecordPlayer* record_player, float** data) {
    uint16_t* dframe = record_player->record_data.frame_data + record_player->idx * record_player->count;
    for (int i = 0; i < record_player->count; i++) {
        record_player->frame[i] = (float) dframe[i] / 10;        
    }
    
    *data = record_player->frame;    
       
    record_player->idx = (record_player->idx + 1) % record_player->record_data.frame_count;
    return true;
}

const float* get_pcl_proj_record_player(RecordPlayer* record_player) {
    return record_player->record_data.projection_data;    
}

