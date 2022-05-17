#version 330 core
out vec4 FragColor;
in vec2 TexCoords;

uniform sampler2D textureSampler;

void main()
{
  vec4 color = vec4(1.0, 1.0, 1.0, texture(textureSampler, TexCoords).r);
  FragColor = color;
}