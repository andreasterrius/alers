#version 330 core
out vec4 FragColor;
in vec2 TexCoords;
in vec4 Color;

void main()
{
  FragColor = Color;
}