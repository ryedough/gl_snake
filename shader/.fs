#version 330
#define SCREEN_H 400
#define SCREEN_W 400
precision mediump float;

uniform vec2 uCircPos;
uniform float uCircRadius;

void main() {
    vec2 circPos = vec2(uCircPos.x, uCircPos.y) ;
    if(length((vec2(gl_FragCoord.x/ SCREEN_W, gl_FragCoord.y / SCREEN_H)) - circPos) > uCircRadius) {
        discard;
    };
    gl_FragColor = vec4(1., 1., 1., 1.0);
}