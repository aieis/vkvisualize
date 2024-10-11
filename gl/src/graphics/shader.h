#include <stdbool.h>

struct ShaderProgram {
    int id;
    char vert_file[1024];
    char frag_file[1024];
};

typedef struct ShaderProgram ShaderProgram;

bool load_shader_program(ShaderProgram* program, const char* vert, const char* frag);
bool reload_shader_program(ShaderProgram* program);
void use_shader(int shader_porgram);
