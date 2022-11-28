#version 140

in vec2 position;
in vec2 texture_position;
in vec4 color;
out vec2 vertex_texture_position;
out vec4 vertex_color;

uniform mat4 matrix;

void main() {
	vertex_texture_position = texture_position;
	vertex_color = color;
	gl_Position = matrix * vec4(position, 0.0, 1.0);
}