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
bool pointInBox(vec2 pos, vec2 boxStart, vec2 boxEnd);

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
        } else {
            // this is tail
            vec2 tailPos = vec2(0,0);
            switch(current.from){
                case UP : tailPos = vec2(current.at.x,current.at.y - nextDst); break;
                case RIGHT : tailPos = vec2(current.at.x - nextDst,current.at.y); break;
                case DOWN : tailPos = vec2(current.at.x,current.at.y + nextDst); break;
                case LEFT : tailPos = vec2(current.at.x + nextDst,current.at.y); break;
            }
            if(pointInRadius(frag_pos, tailPos)){
                gl_FragColor = vec4(1., 1., 1., 1.0);
                return;
            }
        };

        if(nextDst == 0){
            discard;
            return;
        }

        switch(current.from){
            case UP :
                if(pointInBox(frag_pos, 
                    vec2(current.at.x - uCircRadius, current.at.y - nextDst), 
                    vec2(current.at.x + uCircRadius, current.at.y))) {
                        gl_FragColor = vec4(1., 1., 1., 1.0);
                        return;
                }
                break;
            case RIGHT : 
                if(pointInBox(frag_pos, 
                    vec2(current.at.x - nextDst, current.at.y - uCircRadius), 
                    vec2(current.at.x, current.at.y + uCircRadius))) {
                        gl_FragColor = vec4(1., 1., 1., 1.0);
                        return;
                }
                break;
            case DOWN : 
                if(pointInBox(frag_pos, 
                    vec2(current.at.x - uCircRadius, current.at.y), 
                    vec2(current.at.x + uCircRadius, current.at.y + nextDst))) {
                        gl_FragColor = vec4(1., 1., 1., 1.0);
                        return;
                }
                break;
            case LEFT : 
                if(pointInBox(frag_pos, 
                    vec2(current.at.x, current.at.y - uCircRadius), 
                    vec2(current.at.x + nextDst, current.at.y + uCircRadius))) {
                        gl_FragColor = vec4(1., 1., 1., 1.0);
                        return;
                }
                break;
        }
    };
    discard;  
}

bool pointInRadius(vec2 pos, vec2 center) {
    return (length(pos - center) < uCircRadius);
}

// box start must be on bottom left, box's end must be on top right
// e.g :
// boxStart = (0.1, 0.1)
// boxEnd = (0.5, 0.5)
bool pointInBox(vec2 pos, vec2 boxStart, vec2 boxEnd) {
    return 
        (pos.x > boxStart.x && pos.x < boxEnd.x) &&
        (pos.y > boxStart.y && pos.y < boxEnd.y);
}