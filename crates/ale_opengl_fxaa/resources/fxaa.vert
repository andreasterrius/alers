#version 330 core
layout (location = 0) in vec2 aPos;

out vec2 TexCoords;

void main()
{
  // Transform
  vec2 pos = (aPos * 2) - 1;
  TexCoords = aPos;

  gl_Position = vec4(pos, 0.0, 1.0);
}