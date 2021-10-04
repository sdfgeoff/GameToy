// Cavefly State pass. Handles motion/physics/scoring

void main(){
    
    ivec2 addr = ivec2(fragCoord);
    if (addr == ADDR_CAMERA_POSITION) {
        fragColor = vec4(3.0, 3.0 + sin(iTime), 5.0, 1.0);
    }
}
