#include "input.h"

#include <stdbool.h>
#include <stdio.h>

#include "GLFW/glfw3.h"
#include "cglm/cam.h"
#include "cglm/affine.h"
#include "cglm/vec3.h"

void init_mvp(ProgramState* state) {
    glm_mat4_identity(state->model);
    glm_perspective(glm_rad(45.0), (float) state->W / (float) state->H, 0.1, 100, state->projection);
    glm_lookat((vec3) {0.0, 0.0, -2.0}, (vec3) {0.0, 0.0, 0.0}, (vec3) {0.0, -1.0, 0.0}, state->view);
}

void update_mvp(ProgramState* state) {
    mat4 inb;
    glm_mat4_mul(state->view, state->model, inb);
    glm_mat4_mul(state->projection, inb, state->mvp);
}


void make_state(ProgramState* state, int width, int height) {
    state->W = width;
    state->H = height;
    
    init_mvp(state);
    update_mvp(state);
}   

struct MovementShortcuts {
    int key;
    float val;
    int axis;
};

bool key_down(GLFWwindow* window, int key) {
    int key_state = glfwGetKey(window, key);
    return key_state == GLFW_PRESS || key_state == GLFW_REPEAT;
}

void process_events(ProgramState* state, GLFWwindow* window) {

    const float delta = 0.001;
    struct MovementShortcuts movement_shortcuts[6] =
        {
            {GLFW_KEY_W,            -delta, 2},
            {GLFW_KEY_S,             delta, 2},
            {GLFW_KEY_A,             delta, 0},
            {GLFW_KEY_D,            -delta, 0},
            {GLFW_KEY_SPACE,        -delta, 1},
            {GLFW_KEY_LEFT_CONTROL,  delta, 1},
        };

    struct MovementShortcuts rotation_shortcuts[6] =
        {
            {GLFW_KEY_W,             delta, 0},
            {GLFW_KEY_S,            -delta, 0},
            {GLFW_KEY_A,            -delta, 1},
            {GLFW_KEY_D,             delta, 1},
            {GLFW_KEY_SPACE,         delta, 2},
            {GLFW_KEY_LEFT_CONTROL, -delta, 2},
        };
    
    int mouse_button = glfwGetMouseButton(window, GLFW_MOUSE_BUTTON_1);
    bool mouse_down = mouse_button == GLFW_PRESS || mouse_button == GLFW_REPEAT;
    if (!mouse_down) {
        for (int i = 0; i < 6; i++) {
            if (key_down(window, movement_shortcuts[i].key)) {                
                vec3 axis = {};
                axis[movement_shortcuts[i].axis] = movement_shortcuts[i].val;
                glm_translate(state->model, axis);
            }
        }
    } else {
        for (int i = 0; i < 6; i++) {
            if (key_down(window, rotation_shortcuts[i].key)) {
                vec3 axis = {};
                axis[rotation_shortcuts[i].axis] = 1;
                glm_rotate(state->model, rotation_shortcuts[i].val, axis);
            }
        }
    }

    if (key_down(window, GLFW_KEY_Q)) {
        glfwSetWindowShouldClose(window, 1);
    }

    if (key_down(window, GLFW_KEY_T)) {
        init_mvp(state);
    }
    update_mvp(state);
}
