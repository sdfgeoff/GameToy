#version 300 es
// Color screen based on on-screen-position
precision lowp float;
in vec2 fragCoordUV;
out vec4 fragColor;

uniform mediump vec3 iResolution;
uniform sampler2D col;



void main() {
        fragColor = texture(col, fragCoordUV);
}
