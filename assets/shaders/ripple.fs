#version 330

// Input vertex attributes (from vertex shader)
in vec2 fragTexCoord;
in vec4 fragColor;

// Input uniform values
uniform sampler2D texture0;
uniform vec4 colDiffuse;

// Output fragment color
out vec4 finalColor;

// NOTE: Add here your custom variables

const vec2 size = vec2(1080, 720);   // render size
const float samples = 5.0;          // pixels per axis; higher = bigger glow, worse performance
const float quality = 1.5;             // lower = smaller glow, better quality

void main()
{
    vec4 sum = vec4(0);
    vec2 sizeFactor = vec2(1)/size*quality;

    // Texel color fetching from texture sampler
    vec4 source = texture(texture0, fragTexCoord);

    const int range = 2;            // should be = (samples - 1)/2;

    for (int x = -range; x <= range; x++)
    {
        for (int y = -range; y <= range; y++)
        {
            sum += texture(texture0, fragTexCoord + vec2(x, y)*sizeFactor);
        }
    }

    // Calculate final fragment color
    finalColor = ((sum/(samples*samples)) + source)*colDiffuse;
}

// #version 330

// // Input vertex attributes (from vertex shader)
// in vec2 fragCoord;
// out vec4 fragColor;

// vec2      resolution = vec2(1080.0, 720.0);           // viewport resolution (in pixels)
// uniform float     time = 0.0;                 // shader playback time (in seconds)
// // uniform float     iTimeDelta;            // render time (in seconds)
// // uniform int       iFrame;                // shader playback frame
// // uniform float     iChannelTime[4];       // channel playback time (in seconds)
// // uniform vec3      iChannelResolution[4]; // channel resolution (in pixels)
// // uniform vec4      iMouse;                // mouse pixel coords. xy: current (if MLB down), zw: click
// uniform sampler2D texture0;          // input channel. XX = 2D/Cube
// // uniform vec4      iDate;                 // (year, month, day, time in seconds)
// // uniform float     iSampleRate;           // sound sample rate (i.e., 44100)



// void main()
// {
    
//     vec2 uv = fragCoord.xy / vec2(1080.0, 720.0);
    
//     float X = uv.x*25.+time;
//     float Y = uv.y*25.+time;
//     uv.y += cos(X+Y)*0.01*cos(Y);
//     uv.x += sin(X-Y)*0.01*sin(Y);
	
//     fragColor = texture(texture0,uv);
// }
// // #version 330

// // // Input vertex attributes (from vertex shader)
// // in vec2 fragTexCoord;
// // in vec4 fragColor;

// // // Input uniform values
// // uniform sampler2D texture0;
// // uniform vec4 colDiffuse;

// // // Output fragment color
// // out vec4 finalColor;

// // // NOTE: Add here your custom variables

// // void main()
// // {
// //     // Texel color fetching from texture sampler
// //     vec4 texelColor = texture(texture0, fragTexCoord)*colDiffuse*fragColor;
    
// //     // Convert texel color to grayscale using NTSC conversion weights
// //     float gray = dot(texelColor.rgb, vec3(0.299, 0.587, 0.114));
    
// //     // Calculate final fragment color
// //     finalColor = vec4(gray, gray, gray, texelColor.a);
// // }