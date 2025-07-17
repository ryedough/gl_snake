#version 330

uniform vec2 uPosition;
uniform float uRadius;

void main(){  
    if(length(gl_FragCoord.xy - uPosition) < uRadius){
        gl_FragColor=vec4(1., 0.,0.,1.);
    }else {
        discard;
    };
}