#version 330
#define SCREEN_H 400
#define SCREEN_W 400
#define MAX_KEYPOINTS 100

#define UP 0
#define RIGHT 1
#define DOWN 2
#define LEFT 3

precision mediump float;

struct MoveKeypoint {
    uint from;
    vec2 at;
    float dstHead;
};

uniform float uCircRadius;
uniform float uLength;

uniform MoveKeypoint[MAX_KEYPOINTS] uKeypoints;
uniform uint uKeypointLen;

bool pointInRadius(vec2 pos, vec2 center);

void main() {
    vec2 frag_pos = vec2(gl_FragCoord.x/ SCREEN_W, gl_FragCoord.y / SCREEN_H);

    float remainLength = uLength; 

    for(uint _i= uKeypointLen; _i > uint(0); _i--) {
        uint i = _i-uint(1);

        MoveKeypoint current = uKeypoints[i];

        if(pointInRadius(frag_pos, current.at)){
            gl_FragColor = vec4(1., 1., 1., 1.0);
            return;
        }

        //set next dst
        //TODO: set this to remaining snake length when no next keypoint
        float nextDst = remainLength;
        if(_i > uint(1)){
            uint nextIdx = i-uint(1);
            nextDst = uKeypoints[nextIdx].dstHead - current.dstHead;
            remainLength -= nextDst;
        };

        if(nextDst == 0){
            discard;
            return;
        }

        switch(current.from){
            case UP : 
                for(float i = 0; i < nextDst; i+=0.02){
                    if(pointInRadius(frag_pos, vec2(current.at.x, current.at.y - i))){
                        gl_FragColor = vec4(1., 1., 1., 1.0);
                        return;
                    }
                }
                break;
            case RIGHT : 
                for(float i = 0; i < nextDst; i+=0.02){
                    if(pointInRadius(frag_pos, vec2(current.at.x - i, current.at.y))){
                        gl_FragColor = vec4(1., 1., 1., 1.0);
                        return;
                    }
                }
                break;
            case DOWN : 
                for(float i = 0; i < nextDst; i+=0.02){
                    if(pointInRadius(frag_pos, vec2(current.at.x, current.at.y + i))){
                        gl_FragColor = vec4(1., 1., 1., 1.0);
                        return;
                    }
                }
                break;
            case LEFT : 
                for(float i = 0; i < nextDst; i+=0.02){
                    if(pointInRadius(frag_pos, vec2(current.at.x + i, current.at.y))){
                        gl_FragColor = vec4(1., 1., 1., 1.0);
                        return;
                    }
                }
                break;
        }
    };
    discard;  
}

bool pointInRadius(vec2 pos, vec2 center) {
    return (length(pos - center) < uCircRadius);
}