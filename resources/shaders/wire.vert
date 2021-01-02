#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aBarycentric;

uniform mat4 view;
uniform mat4 projection;
uniform mat4 model;

out vec3 Barycentric;

void main(){
    Barycentric = aBarycentric;
    gl_Position = projection * view * model * vec4(aPos, 1.0);
}