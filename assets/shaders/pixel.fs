#version 330

// Input vertex attributes (from vertex shader)
in vec2 fragTexCoord;
in vec4 fragColor;

// Input uniform values
uniform sampler2D texture0;
uniform vec4 colDiffuse;

// Output fragment color
out vec4 finalColor;

// Viewport dimensions
const vec2 viewport = vec2(1080.0, 720.0);
// const float renderWidth = 1080;
// const float renderHeight = 720;

// Pixel scaling
uniform float pixelWidth = 2.0;
uniform float pixelHeight = 2.0;

// Time value, fed from CPU
uniform float time = 0.0;

void main()
{

    // Calculate the pixel to a UV
    vec2 uv = fragTexCoord.xy / viewport.xy;

    float X = uv.x * 25.0 + time;
    float Y = uv.y * 25. + time;
    uv.y += cos(X + Y) * 0.01 * cos(Y);
    uv.x += sin(X-Y) * 0.01 * sin(Y); 

    // Calculate the pixel merge distance
    float dx = pixelWidth * (1.0 / viewport.x);
    float dy = pixelHeight * (1.0 / viewport.y);

    // Use UV to make wavy
    // vec4 tc = texture(texture0, uv);

    // Use UV to pixelate image
    vec2 coord = vec2(dx * floor(fragTexCoord.x / dx), dy * floor(fragTexCoord.y / dy));
    vec3 tc = texture(texture0, coord + uv).rgb;

    // Shift the hue to look like underwater
    tc = tc + vec3(0, 0.05, 0.15);

    finalColor = vec4(tc, 1.0);
    // finalColor = tc;
}
