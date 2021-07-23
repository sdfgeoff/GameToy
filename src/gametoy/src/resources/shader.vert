#version 300 es
precision lowp float;
uniform mediump vec3 iResolution;
in vec2 aVertexPosition;
out vec2 fragCoordUV;
out vec2 fragCoord;

void main() {
        vec2 screen_pos = aVertexPosition * 2.0 - vec2(1.0);
	fragCoordUV = aVertexPosition;
        fragCoord = aVertexPosition * iResolution.xy;
	gl_Position = vec4(
                screen_pos,
                0.0,
                1.0
        );
}
