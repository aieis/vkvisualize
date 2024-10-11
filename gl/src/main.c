#include <stdio.h>

#include "glad/glad.h"
#include "GLFW/glfw3.h"

#include "graphics/shader.h"
#include "graphics/compass.h"
#include "graphics/pcl.h"
#include "state.h"

int main(int argc, char** argv) {
   if (!glfwInit()) {
       fprintf(stderr, "Could not initialize glfw\n");
       return 1;
   }

   glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
   glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 6);
   glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

   WindowState win_state = {512, 512 };
   glm_mat4_identity(win_state.mvp);

   GLFWwindow* window = glfwCreateWindow(win_state.W, win_state.H, "PCL", NULL, NULL);

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

   glEnable(GL_VERTEX_PROGRAM_POINT_SIZE);
            
   ShaderProgram compass_shader;
   if (!load_shader_program(&compass_shader, "assets/shaders/line.vert", "assets/shaders/col.frag")) {
       fprintf(stderr, "Could not create compass shader\n");
       return 1;
   }

   int compass_shader_mvp = glGetUniformLocation(compass_shader.id, "mvp");
      
   ShaderProgram pcl_shader = {};
   if (!load_shader_program(&pcl_shader, "assets/shaders/pcl.vert", "assets/shaders/col.frag")) {
       fprintf(stderr, "Could not create compass shader\n");
       return 1;
   }
   
   int pcl_shader_mvp = glGetUniformLocation(pcl_shader.id, "mvp");
   int pcl_shader_col = glGetUniformLocation(pcl_shader.id, "col");
   
   PointCloud pcl = {};
   make_point_cloud(&pcl, (float[3]) {0.8f, 0.2f, 0.2f});
   update_point_cloud(&pcl, (float[6]) {0.5, 0.5, 0.0, -0.5, -0.5, 0.0}, 2);   

   Compass compass;
   make_compass(&compass);
   glClearColor(0.1f, 0.1f, 0.1f, 1.0f);
   while (!glfwWindowShouldClose(window)) {
       glfwPollEvents();
       
       glClear(GL_COLOR_BUFFER_BIT);

       glUseProgram(compass_shader.id);
       glUniformMatrix4fv(compass_shader_mvp, 1, GL_FALSE, (float*) win_state.mvp);
       draw_compass(&compass, 0);

       glUseProgram(pcl_shader.id);
       glUniformMatrix4fv(pcl_shader_mvp, 1, GL_FALSE, (float*) win_state.mvp);
       glUniform3fv(pcl_shader_col, 1, (float[3]) {0.8f, 0.2f, 0.2f});
       draw_point_cloud(&pcl);

       glfwSwapBuffers(window);
   }

   glfwTerminate();
}
