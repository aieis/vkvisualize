#include "record_player.h"

bool make_record_player(RecordPlayer* record_player, const char* file) {

    if (!read_record_data(record_player->record_data, file)) {
        return false;
    }

    record_player->width = record_player->record_data->width;
    record_player->height = record_player->record_data->height;
    record_player->count = record_player->width * record_player->height;
    
    record_player->idx = 0;
    record_player->projection_data = record_player->record_data->projection_data;
    return true;
}


bool poll_record_player(RecordPlayer* record_player, float** projection_data, uint16_t** data) {
    *projection_data = record_player->projection_data;
    *data = record_player->record_data->frame_data + record_player->idx * record_player->count;
    record_player->idx = (record_player->idx + 1) % record_player->record_data->frame_count;
    return true;
}

