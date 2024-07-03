#version 330

in vec2 position;
in vec4 color;
in uvec2 background_id;
in vec2 background_uvs;

out vec4 fs_color;
flat out uvec2 fs_background_id;
out vec2 fs_background_uvs;

void main() {
	fs_color = color;
	fs_background_id = background_id;
	fs_background_uvs = background_uvs;
	
	gl_Position = vec4(position, 0.0, 1.0);
}