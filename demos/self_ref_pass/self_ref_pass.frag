void main() {
    if (iFrame == 0u) {
        state = vec4(step(0.5, mod(fragCoordUV.x * 10.0, 1.0)));
    } else {
        
        vec2 delta = vec2(1.0/iResolution.x, 0.0);
        vec4 left = texture(prev_state, fragCoordUV + delta);
        vec4 here = texture(prev_state, fragCoordUV);
        vec4 right = texture(prev_state, fragCoordUV - delta);
        state = (left + here + right) / 3.0;
    }
}
