#version 460 core

layout (location = 0) in vec3 pos;
       
uniform mat4 mvp;
uniform vec3 col;

out vec3 vs_color;

void main()
{
    gl_Position = mvp * vec4(pos, 1.0);
    float dist = distance(pos, vec3(0.0, 0.0, 0.0));
    
    if (dist < 0.001) {
        gl_PointSize = 0;
    } else {
        gl_PointSize = 0.01 / dist;        
    }

    gl_PointSize = 5.0;
    vs_color = col;
}
