#include <stdbool.h>

#include "cglm/mat4.h"

struct WindowState {
    int W;
    int H;
    mat4 mvp;
};

typedef struct WindowState WindowState;


struct ProgramState {
    int W;
    int H;
    
    mat4 mvp;
    bool new_mvp;
    
    bool conf;
};
