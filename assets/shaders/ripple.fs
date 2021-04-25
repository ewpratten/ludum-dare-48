// void mainImage( out vec4 fragColor, in vec2 fragCoord )
// {
    
//     vec2 uv = fragCoord.xy / iResolution.xy;
    
//     float X = uv.x*25.+iTime;
//     float Y = uv.y*25.+iTime;
//     uv.y += cos(X+Y)*0.01*cos(Y);
//     uv.x += sin(X-Y)*0.01*sin(Y);
	
//     fragColor = texture(iChannel0,uv);
// }
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

void main()
{
    // Texel color fetching from texture sampler
    vec4 texelColor = texture(texture0, fragTexCoord)*colDiffuse*fragColor;
    
    // Convert texel color to grayscale using NTSC conversion weights
    float gray = dot(texelColor.rgb, vec3(0.299, 0.587, 0.114));
    
    // Calculate final fragment color
    finalColor = vec4(gray, gray, gray, texelColor.a);
}