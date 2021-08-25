// Game State Buffer
//
// This buffer stores the state of the game


// Return the state of a key
float get_key(int key_code) {
    return texelFetch(BUFFER_KEYBOARD, ivec2(key_code,0), 0).x;
}
// Return the state of a key
float get_key_tap(int key_code) {
    return texelFetch(BUFFER_KEYBOARD, ivec2(key_code,1), 0).x;
}


void main()
{
    ivec2 addr = ivec2(fragCoordUV * iResolution.xy);
    
    if (addr == ADDR_RESET) {
        //vec2 prev_resolution = read_data(BUFFER_STATE, ADDR_RESET).gb;
        //vec2 current_resolution = iChannelResolution[1].xy;
        
        //vec2 res_change = prev_resolution - current_resolution;
        
        //fragColor.r = dot(res_change, res_change) + get_key(KEY_ESC);
        //fragColor.gb = current_resolution;
        fragColor.r = get_key(KEY_ESC) + float(iFrame == 0u);
        return;
    }
    
    if (addr == ADDR_MAP_SETTINGS) {
        if (get_key(KEY_ESC) != 0.0) {
            fragColor = vec4(iTime);
        }
        return;
    }
    
    else if (addr == ADDR_CAMERA_POSITION) {
        vec4 prev_position = read_data(BUFFER_STATE, ADDR_CAMERA_POSITION);
        
        vec4 player_state = read_data(BUFFER_STATE, ADDR_PLAYER_STATE);
        vec3 player_position, player_velocity;
        float health, shoot;
        unpack_player(player_state, player_position, player_velocity, health, shoot); 
        
        
        vec4 target_position = vec4(
            player_position.xy + player_velocity.xy,
            CAM_ZOOM + length(player_velocity.xy) * CAM_ZOOM_SCALING,
            0.0
        );
        
        fragColor = mix(target_position, prev_position, vec4(CAM_DAMPING));
        return;
    }
    
    else if (addr == ADDR_PLAYER_STATE) {
        vec4 player_state = read_data(BUFFER_STATE, ADDR_PLAYER_STATE);
        vec3 player_position, player_velocity;
        float health, shoot;
        unpack_player(player_state, player_position, player_velocity, health, shoot); 
        
        float thrust_amt = get_key(KEY_UP) - get_key(KEY_DOWN);
        float rotate_amt = get_key(KEY_LEFT) - get_key(KEY_RIGHT);
        
        vec3 acceleration = vec3(
            -sin(player_position.z) * thrust_amt, 
            cos(player_position.z) * thrust_amt, 
            rotate_amt
        ) * SHIP_ACCELERATION;
        
        acceleration -= player_velocity * SHIP_DAMPING;
        
        player_velocity += acceleration * iTimeDelta;
        player_position += player_velocity * iTimeDelta;
        if (player_position.z > PI) {
            player_position.z -= 2.0 * PI;
        } else if (player_position.z < -PI) {
            player_position.z += 2.0 * PI;
        }
        
        // Collision
        vec4 map_data = map_base(BUFFER_MAP_STATE, player_position.xy);
        if (map_data.b + SHIP_TERRAIN_HITBOX_SIZE > 0.0){
            player_position.xy -= map_data.xy * (map_data.b + SHIP_TERRAIN_HITBOX_SIZE);
            player_velocity.xy = reflect(player_velocity.xy, map_data.xy) * SHIP_TERRAIN_BOUNCE_ELASTICITY;
        }
        
        // Shooting
        shoot += iTimeDelta / BULLET_RELOAD_TIME_SEC;
        if (shoot >= 1.0 && get_key(KEY_SHOOT) != 0.0) {
            shoot = 0.0;
        }
        
        fragColor = pack_player(player_position, player_velocity, health, shoot);
        return;
    }
    
    else if (addr.x == ADDR_BULLET_START.x && addr.y <= MAX_BULLETS) {
        
        vec4 player_state = read_data(BUFFER_STATE, ADDR_PLAYER_STATE);
        vec3 player_position, player_velocity;
        float health, shoot;
        unpack_player(player_state, player_position, player_velocity, health, shoot); 
        
        vec4 bullet_state = read_data(BUFFER_STATE, addr);
        vec3 bullet_position, bullet_velocity;
        float age;
        unpack_bullet(bullet_state, bullet_position, bullet_velocity, age); 
        
        if (shoot == 0.0) {
            
            if (addr.y == 0) {
                bullet_position = player_position;
                bullet_velocity.y = cos(player_position.z) * BULLET_SPEED;
                bullet_velocity.x = -sin(player_position.z) * BULLET_SPEED;
                age = BULLET_LIFE_TIME;
            } else {
                bullet_state = read_data(BUFFER_STATE, addr + ivec2(0,-1));
                unpack_bullet(bullet_state, bullet_position, bullet_velocity, age); 
            }
        }
        
        // TODO: Check Collision
        bullet_position += bullet_velocity * iTimeDelta;
        age = max(age - iTimeDelta, 0.0);
        fragColor = pack_bullet(bullet_position, bullet_velocity, age);
        return;
    }
    
    fragColor = vec4(0.0,0.0,1.0,1.0);
}
