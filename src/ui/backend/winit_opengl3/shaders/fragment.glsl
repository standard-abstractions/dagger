#version 330
#extension GL_ARB_shader_storage_buffer_object : require
#extension GL_ARB_bindless_texture : require

in vec4 fs_color;

out vec4 color;

void main() {
	color = fs_color;
}