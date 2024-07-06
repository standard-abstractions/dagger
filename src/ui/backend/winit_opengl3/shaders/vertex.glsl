#version 330

in vec2 position;
in vec4 color;

out vec4 fs_color;

void main() {
	fs_color = color;
	
	gl_Position = vec4(position, 0.0, 1.0);
}