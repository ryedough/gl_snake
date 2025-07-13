#version 330
precision mediump float;
in vec2 position;
out vec4 color;

void main() {
  color = vec4(position, 1.0, 1.0);
}