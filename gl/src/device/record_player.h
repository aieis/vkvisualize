#include <stdbool.h>
#include "../utils/record.h"


struct RecordPlayer {
    int width;
    int height;
    int count;

    RecordData record_data;
    float* frame;
    
    int idx;
    float frame_time;
    float last_frame_time;

};

typedef struct RecordPlayer RecordPlayer;

bool make_record_player(RecordPlayer* record_player, const char* file);
bool poll_record_player(RecordPlayer* record_player, float** data);
const float* get_pcl_proj_record_player(RecordPlayer* record_player);
