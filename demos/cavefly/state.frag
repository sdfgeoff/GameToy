// Cavefly State pass. Handles motion/physics/scoring

void main(){
    
    ivec2 addr = ivec2(fragCoord);
    if (addr == ADDR_CAMERA_POSITION) {
        fragColor = vec4(0.0, 0.0, 2.0, 1.0);
    }
}
