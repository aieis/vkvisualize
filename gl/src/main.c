#include <stdio.h>

#include "glad/glad.h"
#include "GLFW/glfw3.h"
#include "cglm/cam.h"

#include "graphics/shader.h"
#include "graphics/compass.h"
#include "graphics/pcl.h"
#include "device/record_player.h"
#include "input.h"

int main(int argc, char** argv) {

    RecordPlayer player;
    const char* target_recording = "assets/recordings/record1.rdbin";
    if (!make_record_player(&player, target_recording)) {
        fprintf(stderr, "Could not open player for recording: '%s'\n", target_recording);
        return 1;
    }
    
    if (!glfwInit()) {
        fprintf(stderr, "Could not initialize glfw\n");
        return 1;
    }

    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 6);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    ProgramState state;
    make_state(&state, 1024, 1024);
    
    GLFWwindow* window = glfwCreateWindow(state.W, state.H, "PCL", NULL, NULL);

    if (!window) {
        fprintf(stderr, "Could not create window\n");
        glfwTerminate();
        return 1;
    }

    glfwMakeContextCurrent(window);

    if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress)) {
        fprintf(stderr, "Could not load glad dll\n");
        glfwTerminate();
        return 1;
    }

    glViewport(0, 0, state.W, state.H);

    glEnable(GL_VERTEX_PROGRAM_POINT_SIZE);
    glEnable(GL_DEPTH_TEST);
    
    ShaderProgram compass_shader;
    if (!load_shader_program(&compass_shader, "assets/shaders/line.vert", "assets/shaders/col.frag")) {
        fprintf(stderr, "Could not create compass shader\n");
        return 1;
    }

    int compass_shader_mvp = glGetUniformLocation(compass_shader.id, "mvp");

    ShaderProgram pcl_shader = {};
    if (!load_shader_program(&pcl_shader, "assets/shaders/pcl.vert", "assets/shaders/col.frag")) {
        fprintf(stderr, "Could not create PCL shader\n");
        return 1;
    }

    int pcl_shader_mvp = glGetUniformLocation(pcl_shader.id, "mvp");
    int pcl_shader_col = glGetUniformLocation(pcl_shader.id, "col");

    PointCloud pcl = {};
    make_point_cloud(&pcl, (float[3]) {0.8f, 0.2f, 0.2f});
    const float* proj_data = get_pcl_proj_record_player(&player);
    update_point_cloud_proj(&pcl, proj_data, player.count);
    
    Compass compass;
    make_compass(&compass);
    glClearColor(0.1f, 0.1f, 0.1f, 1.0f);
    while (!glfwWindowShouldClose(window)) {
        glfwPollEvents();
        process_events(&state, window);

        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

        float* data;
        if (poll_record_player(&player, &data)) {
            update_point_cloud(&pcl, data, player.count);
        }

        glUseProgram(pcl_shader.id);
        glUniformMatrix4fv(pcl_shader_mvp, 1, GL_FALSE, (float*) state.mvp);
        glUniform3fv(pcl_shader_col, 1, (float[3]) {0.8f, 0.2f, 0.2f});
        draw_point_cloud(&pcl);

        glUseProgram(compass_shader.id);
        glUniformMatrix4fv(compass_shader_mvp, 1, GL_FALSE, (float*) state.mvp);
        draw_compass(&compass, 0);


        glfwSwapBuffers(window);
    }

    glfwTerminate();
}
