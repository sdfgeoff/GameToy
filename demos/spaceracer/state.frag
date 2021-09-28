// Contains gameplay state

float get_key(int key_code) {
    return texelFetch(CHANNEL_KEYS, ivec2(key_code,0), 0).x;
}

float wrap_angle(float angle) {
    // Ensure a number is between pi and -pi
    // Not sure if this is the optimal way, but it
    // avoids issues with fract/mod indeterminism on
    // negative numbers
    angle = angle + PI; // Work between 0 and 2PI;
 	float sig = sign(angle);
    float mag = mod(abs(angle), 2.0 * PI);
    
    return sig * (mag - PI);
}


void main()
{
    ivec2 address = ivec2(fragCoord);
    
    if (address == A_SHIP_1 || address == A_SHIP_2 || address == A_SHIP_3 || address == A_SHIP_4) {
        // Data about the ship: Texture sample 1
        ship_t ship = unpack_ship(read_data(CHANNEL_STATE, address));
        
        // Data about what is underneath the ship: Texture sample 2
        vec4 map_data = sample_map(CHANNEL_MAP, CHANNEL_MAP_RESOLUTION, ship.position.xy);
        
        
        if (iTime < STARTING_DELAY) {
            ship.velocity = vec3(0.0);
            
            vec3 start_offset = vec3(0.15, 0.15, 0.0) - vec3(address, 0.0) * 0.3;
            float c = cos(MAP_START_LINE.z);
            float s = sin(MAP_START_LINE.z);
            mat3 ori = mat3(
                c, s, 0.0,
                -s, c, 0.0,
                0.0, 0.0, 1.0
            );
            
            ship.position = MAP_START_LINE + ori * start_offset;
            ship.flame = false;
            
            fragColor = pack_ship(ship);
            return;
        }
        
        float c = cos(ship.position.z);
        float s = sin(ship.position.z);
        mat2 ori = mat2(
            c, s,
            -s, c
        );
        vec2 forwards = ori * vec2(0.0, 1.0);
        
        float thrust = 0.0;
        float steer = 0.0;
        
        
        // Compute control for this ship. For the player (ship 1) it is keyboard
        // anything else is AI
        if (address == A_SHIP_1) {
            float thrust_keys = get_key(KEY_UP) - get_key(KEY_DOWN);
            ship.flame = thrust_keys > 0.0; // Indicate that thrust is being applied to the ship
			thrust = thrust_keys;
            steer = (get_key(KEY_LEFT) - get_key(KEY_RIGHT));

        } else {
            // AI using map SDF/metadata
            
            // This is used to make the different ships behave differently.
            float diff = float(address.x) + float(address.y) * 2.0;
            
            thrust = 1.0;
            
            vec2 corrective_direction = map_data.xy * vec2(1.0, -1.0); // Points towards track center
            float course_angle = atan(-ship.position.y, -ship.position.x); // Go around track clockwise
            vec2 course_direction = vec2(sin(course_angle), cos(course_angle));
            
            float off_course_amount = 0.5;
            
            vec2 target_direction = mix(course_direction, corrective_direction, off_course_amount);
            float target_angle = atan(target_direction.x, target_direction.y);
            
            steer = target_angle - ship.position.z;

            steer = wrap_angle(steer);
            steer *= (0.12 - map_data.z) * pow(3.0, diff);

            //steering -= ship.velocity.z * diff * 0.2; // Some damping - doesn't help.
            
            //ship.velocity.z += steering;
            
            ship.flame = thrust > 0.0;
        }
        
        vec3 acceleration = vec3(0.0);
        
        acceleration.xy += forwards * clamp(thrust, -1.0, 1.0);
        acceleration.z += steer;
        acceleration *= SHIP_THRUST;
        
        // Damping/friction
        acceleration -= ship.velocity * SHIP_DAMPING;
        
        // Collision with map
        float ship_size = SHIP_SCALE / MAP_SCALE;
        if (map_data.b > -ship_size) {
            // Compute vector to back on the course
            vec2 overlap = (map_data.xy) * (map_data.z + ship_size) * MAP_SCALE;
            ship.position.xy -= overlap * 0.5;
            acceleration.xy -= ship.velocity.xy * GROUND_FRICTION;
        }
        
        
        // Integration
        ship.velocity += acceleration * iTimeDelta;
        ship.position += ship.velocity * iTimeDelta;
        ship.position.z = wrap_angle(ship.position.z);
        
        fragColor = pack_ship(ship);
    }
}