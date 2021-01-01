#version 330 core

in vec3 Barycentric;
out vec4 FragColor;

void main()
{
    if(any(lessThan(Barycentric, vec3(0.01)))){
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    }
    else{
        gl_FragColor = vec4(0.0, 0.0, 0.0, 0.0);
    }
}