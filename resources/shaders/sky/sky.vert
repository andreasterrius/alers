#version 330 core
layout(location = 0) in vec4 aPos;

uniform mat4 model;
uniform mat4 view;

out vec3 viewRay;

void main(){
  viewRay = (model * view * aPos).xyz;
  gl_Position = vertex;
}