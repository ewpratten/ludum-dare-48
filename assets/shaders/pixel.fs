#version 330

// Input vertex attributes (from vertex shader)
in vec2 fragTexCoord;
in vec4 fragColor;

// Input uniform values
uniform sampler2D texture0;
uniform vec4 colDiffuse;

// Time fed from CPU
uniform float time = 0.0;

// Output fragment color
out vec4 finalColor;

// Viewport dimensions
const vec2 viewport = vec2(1080.0, 720.0);

// Pixel scaling
const vec2 pixelScale = vec2(2.0, 2.0);

void main()
{

    // Calculate the distance to merge pixels
    float dx = pixelScale.x * (1.0 / viewport.x);
    float dy = pixelScale.y * (1.0 / viewport.y);

    // Get the base UV coordinate of the pixel
    vec2 baseUV = fragTexCoord;
    
    // Use a wave function to translate the pixel UV
    float X = baseUV.x*5.+time;
    float Y = baseUV.y*5.+time;
    baseUV.y += cos(X+Y)*0.01*cos(Y);
    baseUV.x += sin(X-Y)*0.01*sin(Y);

    // Calculate a UV for this new blocky pixel
    vec2 pixelatedUV = vec2(dx * floor(baseUV.x / dx), dy * floor(baseUV.y / dy));

    // Rebuild the texture with the new UVs
    vec3 tc = texture(texture0, pixelatedUV).rgb;

    // Apply a color filter
    tc = tc + vec3(0, 0.05, 0.15);

    // Build the final pixel
    finalColor = vec4(tc, 1.0);
}