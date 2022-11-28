#version 140

in vec2 vertex_texture_position;
in vec4 vertex_color;
out vec4 color;

uniform sampler2D texture_sampler;

void main() {
	vec4 texture_color = texture(texture_sampler, vertex_texture_position);
	color = mix(vertex_color, texture_color, texture_color.w);
}