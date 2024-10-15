#include "GLFW/glfw3.h"
#include "state.h"
void make_state(ProgramState* state, int width, int height);
void process_events(ProgramState* state, GLFWwindow* window);
void process_events_cb(GLFWwindow* window, int key, int scancode, int action, int mods);
