#version 330

uniform vec2 uPosition;
uniform float uRadius;
uniform float uTime;

void main(){  
    if(length(gl_FragCoord.xy - uPosition) < uRadius - (0.5 * uTime)){
        gl_FragColor=vec4(
         1. * max(uTime,(1-(length(gl_FragCoord.xy - uPosition) / uRadius))),
         1.,
         1. * max(uTime,(1-(length(gl_FragCoord.xy - uPosition) / uRadius))),
         1.);
    }else {
        discard;
    };
}