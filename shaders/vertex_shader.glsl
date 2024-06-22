#version 330

in vec2 position;
in vec4 color;
in uvec2 texture_id;
in vec2 texture_uvs;

out vec4 fs_color;
flat out uvec2 fs_texture_id;
out vec2 fs_texture_uvs;

void main() {
	fs_color = color;
	fs_texture_id = texture_id;
	fs_texture_uvs = texture_uvs;
	
	gl_Position = vec4(position, 0.0, 1.0);
}