#version 330 core
out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D textureSampler;
uniform float contrastThreshold;
uniform float relativeThreshold;

struct PixelData {
    float m, n, e, s, w;
    float ne, nw, se, sw;
    float highest, lowest, contrast;
};

vec3 Saturate(vec3 color){
    return clamp(color, 0.0, 1.0);
}

float LinearRgbToLuminance(vec3 linearRgb) {
	return dot(linearRgb, vec3(0.2126729f,  0.7151522f, 0.0721750f));
}

float SampleLuminance(vec2 uv, int plusU, int plusV){
    vec3 ori = vec3(textureOffset(textureSampler, uv, ivec2(plusU, plusV)));
    return LinearRgbToLuminance(Saturate(ori));
}

float DeterminePixelBlendFactor(PixelData p){
    float f = 2 * (p.n + p.e + p.s + p.w);
    f += p.ne + p.nw + p.se + p.sw;
    f *= 1.0 / 12;
    return f;
}

PixelData SampleLuminanceNeighborhood(vec2 uv){
    PixelData p;
    p.m = SampleLuminance(uv, 0, 0);
    p.n = SampleLuminance(uv, 0, 1);
    p.s = SampleLuminance(uv, 0, -1);
    p.e = SampleLuminance(uv, 1, 0);
    p.w = SampleLuminance(uv, -1, 0);

    p.ne = SampleLuminance(uv, 1, 1);
    p.nw = SampleLuminance(uv, -1, 1);
    p.se = SampleLuminance(uv, 1, -1);
    p.sw = SampleLuminance(uv, -1, -1);

    p.highest = max(max(max(max(p.n, p.e), p.s), p.w), p.m);
    p.lowest = min(min(min(min(p.n, p.e), p.s), p.w), p.m);
    p.contrast = p.highest - p.lowest;

    return p;
}

vec3 FXAA(vec2 uv){
    PixelData p = SampleLuminanceNeighborhood(uv);

    if(p.contrast < relativeThreshold * p.highest) {
        return vec3(0.0, 0.0, 0.0);
    }

    if(p.contrast < contrastThreshold) {
        return vec3(0.0, 0.0, 0.0);
    }

    float pixelBlend = DeterminePixelBlendFactor(p);

    return vec3(p.contrast);
}

void main()
{
    // Calculate luminance data
    vec3 lumi = vec3(FXAA(TexCoords.st));

    // Uncomment for original color
    //lumi = vec3(texture(textureSampler, TexCoords.st));

    FragColor = vec4(lumi, 1.0);
}