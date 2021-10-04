// Cavefly State pass. Handles motion/physics/scoring



float get_key(int key_code) {
    return texelFetch(BUFFER_KEYBOARD, ivec2(key_code,0), 0).x;
}


void main(){
    
    ivec2 addr = ivec2(fragCoord);
    if (addr == ADDR_CAMERA_POSITION) {
        vec4 prev_camera_state = read_data(BUFFER_STATE, ADDR_CAMERA_POSITION);
        
        vec4 player_state = read_data(BUFFER_STATE, ADDR_PLAYER_STATE);
        vec3 player_position, player_velocity;
        float flame, fuel;
        unpack_player(player_state, player_position, player_velocity, flame, fuel);
        
        vec4 target_camera_state = vec4(player_position.xy, 3.0, 0.0);
        
        vec4 camera_state = mix(prev_camera_state, target_camera_state, iTimeDelta); // TODO Use time correct lerp
        
        fragColor = camera_state;
    }
    
    if (addr == ADDR_RESET) {
        fragColor = vec4(0.0, 0.0, 0.0, 0.0);
    }
    if (addr == ADDR_MAP_SETTINGS) {
        fragColor = vec4(0.0, 0.0, 0.0, 0.0);
    }
    
    if (addr == ADDR_PLAYER_STATE) {
        vec4 player_state = read_data(BUFFER_STATE, ADDR_PLAYER_STATE);
        vec3 player_position, player_velocity;
        float flame, fuel;
        unpack_player(player_state, player_position, player_velocity, flame, fuel); 
        
        float thrust_query = get_key(KEY_UP);
        
        flame = thrust_query; // TODO: Add engine throttle response rate
        
        float thrust_amt = flame;
        float rotate_amt = get_key(KEY_LEFT) - get_key(KEY_RIGHT);
        
        vec3 acceleration = vec3(
            -sin(player_position.z) * thrust_amt, 
            cos(player_position.z) * thrust_amt, 
            rotate_amt
        ) * SHIP_ACCELERATION;
        
        acceleration -= player_velocity * SHIP_DAMPING;
        acceleration += SHIP_GRAVITY;
        
        player_velocity += acceleration * iTimeDelta;
        player_position += player_velocity * iTimeDelta;
        if (player_position.z > PI) {
            player_position.z -= 2.0 * PI;
        } else if (player_position.z < -PI) {
            player_position.z += 2.0 * PI;
        }
        
        
        
        fragColor = pack_player(player_position, player_velocity, flame, fuel);
        return;
    }
}
