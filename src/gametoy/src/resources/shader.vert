#version 300 es
precision highp float;
in vec2 aVertexPosition;

in vec2 iResolution;
out vec2 FragCoordUV;

void main() {
        vec2 screen_pos = aVertexPosition * 2.0 - vec2(1.0);
	FragCoordUV = aVertexPosition;
	gl_Position = vec4(
                screen_pos,
                0.0,
                1.0
        );
}
