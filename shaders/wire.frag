#version 330 core

in vec3 Barycentric;
out vec4 FragColor;

uniform float wire_thickness;

void main()
{
    if(any(lessThan(Barycentric, vec3(wire_thickness)))){
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    }
    else{
        gl_FragColor = vec4(0.0, 0.0, 0.0, 0.0);
    }
}