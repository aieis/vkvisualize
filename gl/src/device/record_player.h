#include <stdbool.h>
#include "../utils/record.h"


struct RecordPlayer {
    int width;
    int height;
    int count;

    RecordData* record_data;
    float* projection_data;

    int idx;
    float frame_time;
    float last_frame_time;

};

typedef struct RecordPlayer RecordPlayer;

bool make_record_player(RecordPlayer* record_player, const char* file);
bool poll_record_player(RecordPlayer* record_player, float** projection_data, uint16_t** data);
