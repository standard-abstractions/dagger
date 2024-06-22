#version 330
#extension GL_ARB_shader_storage_buffer_object : require
#extension GL_ARB_bindless_texture : require

in vec4 fs_color;
flat in uvec2 fs_texture_id;
in vec2 fs_texture_uvs;

out vec4 color;

layout(std430) buffer textures_buffer {
	sampler2D textures[];
};


void main() {
	if (fs_texture_id[0] > uint(0)) {
		sampler2D tex = textures[fs_texture_id[1]];
		color = fs_color * texture(tex, fs_texture_uvs);
	} else {
		color = fs_color;
	}
}