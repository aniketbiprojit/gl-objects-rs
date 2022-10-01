#shader vertex

in vec2 in_position;
out vec2 position;
uniform mat4 u_proj_matrix;
void main() {
    position = in_position;
    
    gl_Position = vec4(in_position-0.5, 0.0, 1.0);
}


#shader fragment

precision mediump float;
in vec2 position;
out vec4 color;

void main() {
    color = vec4((1.0, 0.5, 1.0, 1.0));
}