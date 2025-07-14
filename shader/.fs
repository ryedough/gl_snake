#version 330
precision mediump float;

in vec3 color;

void main() {
  gl_FragColor = vec4(color, 1.0);
}