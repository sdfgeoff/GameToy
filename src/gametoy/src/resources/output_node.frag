#version 300 es
// Color screen based on on-screen-position
precision lowp float;
in vec2 FragCoordUV;
out vec4 FragColor;

uniform vec3 iResolution;
uniform sampler2D col;



void main() {
        FragColor = texture(col, FragCoordUV);
}
