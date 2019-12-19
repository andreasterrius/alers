#version 330 core
out vec4 FragColor;

in mat4 model;

uniform sampler2D equirectMap;

const vec2 invAtan = vec2(0.1591, 0.3183);
vec2 SampleSphericalMap(vec3 v) {
    vec2 uv = vec2(atan(v.z, v.x), asin(v.y));
    uv *= invAtan;
    uv += 0.5;
    return uv;
}

void main(){
    vec3 localPos = vec3(model * vec4(0.0));
    vec2 uv = SampleSphericalMap(normalize(localPos));
    vec3 color = texture(equirectMap, uv).rgb;

    FragColor = vec4(color, 1.0);
}