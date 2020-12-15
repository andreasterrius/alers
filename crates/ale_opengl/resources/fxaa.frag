#version 330 core
out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D textureSampler;
uniform float fxaa_contrast_threshold;
uniform float fxaa_relative_threshold;
uniform bool fxaa_is_enabled;

struct PixelData {
    float m, n, e, s, w;
    float ne, nw, se, sw;
    float highest, lowest, contrast;
};

struct EdgeData {
    bool isHorizontal;
    float pixelStep;
};

vec3 Saturate(vec3 color){
    return clamp(color, 0.0, 1.0);
}

float SaturateFloat(float f){
    return clamp(f, 0.0, 1.0);
}

float LinearRgbToLuminance(vec3 linearRgb) {
	return dot(linearRgb, vec3(0.2126729f,  0.7151522f, 0.0721750f));
}

float SampleLuminance(vec2 uv, int plusU, int plusV){
    vec3 ori = vec3(textureOffset(textureSampler, uv, ivec2(plusU, plusV)));
    return LinearRgbToLuminance(Saturate(ori));
}

EdgeData DetermineEdge(PixelData p, vec2 texelSize){
    EdgeData e;
    float horizontal =
        abs(p.n + p.s - 2 * p.m) * 2 +
        abs(p.ne + p.se - 2 * p.e) +
        abs(p.nw + p.sw - 2 * p.w);
    float vertical =
        abs(p.e + p.w - 2 * p.m) * 2 +
        abs(p.ne + p.nw - 2 * p.n) +
        abs(p.se + p.sw - 2 * p.s);
    e.isHorizontal = horizontal >= vertical;

    float pLuminance = e.isHorizontal ? p.n : p.e;
    float nLuminance = e.isHorizontal ? p.s : p.w;
    float pGradient = abs(pLuminance - p.m);
    float nGradient = abs(nLuminance - p.m);

    e.pixelStep = e.isHorizontal ? texelSize.y : texelSize.x;
    if (pGradient < nGradient) {
        e.pixelStep = -e.pixelStep;
    }
    
    return e;
}

float DeterminePixelBlendFactor(PixelData p){
    float f = 2 * (p.n + p.e + p.s + p.w);
    f += p.ne + p.nw + p.se + p.sw;
    f *= 1.0 / 12;
    f = abs(f - p.m);
    f = SaturateFloat(f / p.contrast);
    float blendFactor = smoothstep(0, 1, f);
    return blendFactor * blendFactor;
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

bool ShouldSkipPixel(PixelData p){
    float threshold = max(fxaa_contrast_threshold, fxaa_relative_threshold * p.highest);
    return p.contrast < threshold;
}

vec3 FXAA(vec2 uv, vec2 texelSize){
    PixelData p = SampleLuminanceNeighborhood(uv);

    if(ShouldSkipPixel(p)){
        return textureLod(textureSampler, uv, 0.0).rgb;
    }

    float pixelBlend = DeterminePixelBlendFactor(p);
    EdgeData e = DetermineEdge(p, texelSize);

    if (e.isHorizontal) { uv.y += e.pixelStep * pixelBlend; }
    else { uv.x += e.pixelStep * pixelBlend; }

    return textureLod(textureSampler, uv, 0.0).rgb;
    //return e.pixelStep < 0 ? vec3(1, 0, 0) : vec3(1);
    //return e.isHorizontal ? vec3(1, 0, 0) : vec3(1);
    //return vec3(pixelBlend);
}

void main()
{
    // Calculate luminance data
    ivec2 textureSize = textureSize(textureSampler,0);
    vec2 texelSize = vec2(1.0/float(textureSize.x),1.0/float(textureSize.y));

    vec3 lumi = FXAA(TexCoords.st, texelSize).rgb;
    vec3 color = textureLod(textureSampler, TexCoords.st, 0.0).rgb;

    vec3 finalColor = mix(color, lumi, fxaa_is_enabled);
    FragColor = vec4(finalColor, 1.0);
    //FragColor = vec4(texelSize.x, texelSize.y, 0.0, 1.0);
}