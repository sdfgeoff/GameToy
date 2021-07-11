#version 300 es
precision mediump float;
in vec2 aVertexPosition;

out vec2 screen_pos;

void main() {
	screen_pos = aVertexPosition;
	gl_Position = vec4(
                screen_pos * 2.0 - vec2(1.0),
                0.0,
                1.0
        );
}
