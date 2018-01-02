#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

out vec2 TexCoord;

uniform bool chaos;
uniform bool confuse;
uniform bool shake;
uniform float time;

void main() {
  vec3 remapped = (aPos - 0.5) * 2.0;
  gl_Position = vec4(remapped.x, remapped.y, remapped.z, 1.0);

  if(chaos){
    float strength = 0.3;
    vec2 pos = vec2(aTexCoord.x + sin(time) * strength, aTexCoord.y + cos(time) * strength);
    TexCoord = pos;
  }
  else if (confuse) {
      TexCoord = vec2(1.0 - aTexCoord.x, 1.0 - aTexCoord.y);
  }
  else {
      TexCoord = vec2(aTexCoord.x, aTexCoord.y);
  }

  if(shake){
      float strength = 0.01;
      gl_Position.x += cos(time * 10) * strength;
      gl_Position.y += cos(time * 15) * strength;

  }

}