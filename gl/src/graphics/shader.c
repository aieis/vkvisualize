#include "shader.h"

#include <string.h>
#include <stdlib.h>
#include <stdio.h>

#include "glad/glad.h"

bool string_concat(char* out, const int max_size, const char** strings, const int n);
bool slurp_file(char** buf, const char* filepath);
bool load_shader(int* shader, const char* filepath, int shader_type);

bool load_shader_program(ShaderProgram* program, const char* vert, const char* frag) {

    int vert_sz = strlen(vert);
    if (vert_sz + 1 > 1024) {
        return false;
    }

    memcpy(program->vert_file, vert, vert_sz + 1);
    
    int frag_sz = strlen(frag);
    if (frag_sz + 1 > 1024) {
        return false;
    }

    memcpy(program->frag_file, frag, frag_sz + 1);

    return reload_shader_program(program);
}


bool reload_shader_program(ShaderProgram* program) {
    int vert;
    if (!load_shader(&vert, program->vert_file, GL_VERTEX_SHADER)) {
        fprintf(stderr, "Could not load shader: '%s'\n", program->vert_file);
        return false;
    }

    int frag;
    if (!load_shader(&frag, program->frag_file, GL_FRAGMENT_SHADER)) {
        glDeleteShader(vert);
        fprintf(stderr, "Could not load shader: '%s'\n", program->frag_file);
        return false;
    }

    int prog = glCreateProgram();
    glAttachShader(prog, vert);
    glAttachShader(prog, frag);
    glLinkProgram(prog);

    glDeleteShader(vert);
    glDeleteShader(frag);

    int status;
    glGetProgramiv(prog, GL_LINK_STATUS, &status);
    char log[512];
    if (!status) {
        int read;
        glGetProgramInfoLog(prog, 512, &read, log);
        log[read == 512? read - 1 : read] = 0;
        fprintf(stderr, "ERROR: '%s'\n", log);

        glDeleteProgram(prog);
        return false;
    }

    program->id = prog;
    return true;
}

bool load_shader(int* shader, const char* filepath, int shader_type) {
    char* data = NULL;
    char log[512];

    
    int lshader = glCreateShader(shader_type);
    
    if (lshader == 0) {
        return false;
    }
    
    if (!slurp_file(&data, filepath)) {
        glDeleteShader(lshader);
        return false;
    }
    
    glShaderSource(lshader, 1, &data, NULL);
    glCompileShader(lshader);

    int status;
    glGetShaderiv(lshader, GL_COMPILE_STATUS, &status);
    if (!status) {
        int read;
        glGetShaderInfoLog(lshader, 512, &read, log);
        log[read == 512? read - 1 : read] = 0;
        fprintf(stderr, "ERROR: '%s'\n", log);
        glDeleteShader(lshader);
        free(data);
        return false;
    }

    free(data);    

    *shader = lshader;
    return true;
}

bool slurp_file(char** buf, const char* filepath) {
    FILE *fp = fopen(filepath, "r");
    
    if (fp == NULL) {
        return false;
    }
    if (fseek(fp, 0L, SEEK_END) != 0) {
        fclose(fp);
        return false;
    }
    
    long bufsize = ftell(fp);
    if (bufsize == -1) {
        return false;
    }

    char* source = malloc(sizeof(char) * (bufsize + 1));
    
    if (fseek(fp, 0L, SEEK_SET) != 0) {
        fclose(fp);
        free(source);
        return false;
    }
    
    size_t num_read = fread(source, sizeof(char), bufsize, fp);
    if (num_read == 0) {
        fclose(fp);
        free(source);
        return false;
    }
    
    source[num_read] = 0;
    *buf = source;
    fclose(fp);
    return true;
}



bool string_concat(char* out, const int max_size, const char** strings, const int n) {
    int current_offset = 0;
    for (int i =0; i < n; i++) {
        int a = strlen(strings[i]);
        if (current_offset + a > max_size - 1) {
            return false;
        }
        memcpy(out + current_offset, strings[i], a * sizeof(char));
        current_offset += a;
    }
    out[current_offset] = 0;

    return true;
}
