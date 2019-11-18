#version 330 core
out vec4 FragColor;
in vec2 TexCoord;

uniform vec4 color;
uniform sampler2D texture0;
uniform sampler2D texture1;

void main() {
   FragColor = color * texture(texture0, TexCoord);
}