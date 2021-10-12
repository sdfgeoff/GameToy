// Cavefly State pass. Handles motion/physics/scoring



float get_key(int key_code) {
    return texelFetch(BUFFER_KEYBOARD, ivec2(key_code,0), 0).x;
}
float get_key_edge(int key_code) {
    return texelFetch(BUFFER_KEYBOARD, ivec2(key_code,1), 0).x;
}


void main(){
    
    ivec2 addr = ivec2(fragCoord);
    if (addr == ADDR_CAMERA_POSITION) {
        vec4 prev_camera_state = read_data(BUFFER_STATE, ADDR_CAMERA_POSITION);
        
        vec4 player_state = read_data(BUFFER_STATE, ADDR_PLAYER_STATE);
        vec3 player_position, player_velocity;
        float flame, fuel;
        unpack_player(player_state, player_position, player_velocity, flame, fuel);
        
        vec4 target_camera_state = vec4(player_position.xy, CAMERA_DEFAULT_ZOOM, 0.0);
        
        vec4 camera_state = tlerp(prev_camera_state, target_camera_state, 0.5, iTimeDelta);
        
        if (read_data(BUFFER_STATE, ADDR_RESET).r == 1.0) {
            
            vec2 start_position;
            vec2 light_pos_array[NUM_LIGHTS];
            vec3 light_array[NUM_LIGHTS+1];
            
            vec4 map_metadata = read_data(BUFFER_MAP_STATE, ADDR_MAP_METADATA);
            unpack_map_metadata(map_metadata, start_position, light_pos_array);
            
            
            camera_state = vec4(start_position, 0, 0);
        }
        fragColor = camera_state;
        return;
    }
    
    if (addr == ADDR_RESET) {
        vec4 prev_state = read_data(BUFFER_STATE, ADDR_RESET);
        
        if (get_key_edge(KEY_ESC) > 0.0  || iFrame == 0u) {
            fragColor = vec4(0.0, prev_state.g + 1.0, 0.0, 0.0);
        } else {
            fragColor = prev_state + vec4(1.0, 0.0, 0.0, 0.0);
        }
        return;
    }
    
    if (addr == ADDR_PLAYER_STATE) {
        vec4 player_state = read_data(BUFFER_STATE, ADDR_PLAYER_STATE);
        vec3 player_position, player_velocity;
        float flame, fuel;
        unpack_player(player_state, player_position, player_velocity, flame, fuel); 
        
        float thrust_query = get_key(KEY_UP);
        flame = tlerp(flame, thrust_query, 0.05, iTimeDelta);
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
        
        // Physics with map
        {
            float df_here = sample_map_distance_field(BUFFER_MAP_STATE, ShapeTexture, player_position.xy).b - 0.5;
            if (df_here > 0.0) {
                float df_right = sample_map_distance_field(BUFFER_MAP_STATE, ShapeTexture, player_position.xy + vec2(0.01, 0.0)).b - 0.5;
                float df_above = sample_map_distance_field(BUFFER_MAP_STATE, ShapeTexture, player_position.xy + vec2(0.0, 0.01)).b - 0.5;
            
                vec2 dir =  normalize(vec2(
                    df_here - df_right,
                    df_here - df_above
                )) ;
                player_position.xy += dir * df_here;
                player_velocity.xy = reflect(player_velocity.xy, dir) * SHIP_BOUNCE;
            }
        }
        // Physics with landing pad
        {
            vec2 start_position;
            vec2 light_pos_array[NUM_LIGHTS];
            vec4 map_metadata = read_data(BUFFER_MAP_STATE, ADDR_MAP_METADATA);
            unpack_map_metadata(map_metadata, start_position, light_pos_array);
            float height_above_pad = player_position.y - start_position.y;
            if ((abs(player_position.x - start_position.x) < 0.5) && (height_above_pad < 0.0) && (height_above_pad > -0.5) ) {
                player_velocity *= 0.5;
            }
        }
        
        if (read_data(BUFFER_STATE, ADDR_RESET).r == 1.0) {
            vec2 start_position;
            vec2 light_pos_array[NUM_LIGHTS];
            vec3 light_array[NUM_LIGHTS+1];
            
            vec4 map_metadata = read_data(BUFFER_MAP_STATE, ADDR_MAP_METADATA);
            unpack_map_metadata(map_metadata, start_position, light_pos_array);
            
            player_position = vec3(start_position, 0);
            player_velocity = vec3(0);
            flame = 0.0;
            fuel = 1.0;
        }
        
        fragColor = pack_player(player_position, player_velocity, flame, fuel);
        return;
    }
}
