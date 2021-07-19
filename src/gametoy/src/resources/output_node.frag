#version 300 es
// Color screen based on on-screen-position

precision lowp float;
in vec2 screen_pos;
out vec4 FragColor;


uniform sampler2D col;



void main() {
	FragColor = texture(col, screen_pos);
}
