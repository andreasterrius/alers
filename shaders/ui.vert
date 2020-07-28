#version 330 core
layout (location = 0) in vec2 aPos;

out vec2 TexCoords;

uniform vec4 possize;
uniform mat4 projection;

void main()
{
  // Transform
  vec2 pos = aPos * possize.zw + possize.xy;
  TexCoords = aPos;

  gl_Position = projection * vec4(pos, 0.0, 1.0);
}