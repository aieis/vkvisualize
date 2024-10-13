#include <stdbool.h>

#include "cglm/mat4.h"

struct ProgramState {
    int W;
    int H;
    
    mat4 model;
    mat4 projection;
    mat4 view;
    mat4 mvp;
    
    bool conf;
};
typedef struct ProgramState ProgramState;
