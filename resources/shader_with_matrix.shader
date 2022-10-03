#shader vertex
in vec2 in_position;
out vec4 position;

uniform mat4 u_proj_matrix;

void main() {
  position = u_proj_matrix*vec4(in_position, 0.5, 1.0);
  
  gl_Position = u_proj_matrix*vec4(in_position, 0.0, 1.0);
}

#shader fragment

precision mediump float;
in vec4 position;
out vec4 color;
uniform float blue;

void main() {
  color = vec4(position);
}