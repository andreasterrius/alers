#version 330 core
out vec4 FragColor;
in vec2 TexCoord;

uniform sampler2D texture0;
uniform vec2      offsets[9];
uniform int       edge_kernel[9];
uniform float     blur_kernel[9];
uniform bool chaos;
uniform bool confuse;
uniform bool shake;

void main() {

   FragColor = vec4(0.0f);
   vec3 sample[9];

   if(chaos || shake) {
    for(int i = 0; i < 9; i++)
      sample[i] = vec3(texture(texture0, TexCoord.st + offsets[i]));
   }

    if(chaos)
    {
        for(int i = 0; i < 9; i++)
            FragColor += vec4(sample[i] * edge_kernel[i], 0.0f);
        FragColor.a = 1.0f;
    }
    else if(confuse)
    {
        FragColor = vec4(1.0 - texture(texture0, TexCoord).rgb, 1.0);
    }
    else if(shake)
    {
      for(int i = 0; i < 9; ++i){
        sample[i] = vec3(texture(texture0, TexCoord.st + offsets[i]));
      }

      for(int i = 0; i < 9; i++)
           FragColor += vec4(sample[i] * blur_kernel[i], 0.0f);
       FragColor.a = 1.0f;
   }
   else {
      FragColor = texture(texture0, TexCoord);
   }

}