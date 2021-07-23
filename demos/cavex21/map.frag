// Map State Buffer

// This contains the state of the map. There are some utility functions in 
// the common buffer to aid in sampling this.
//
// Red channel - occupancy


// Hash function from https://www.shadertoy.com/view/4djSRW
vec3 hash33( vec3 p3 ) {
    p3 = fract(p3 * vec3(.1031, .1030, .0973));
    p3 += dot(p3, p3.yxz+33.33);
    return fract((p3.xxy + p3.yxx)*p3.zyx);
}


vec4 gen_map(vec2 fragCoord, vec4 map_settings) {
    
    // Fill the map density with random noise:
    ivec2 coord = ivec2(fragCoord);

    
    if (any(greaterThan(coord, MAP_SIZE))) {
        return vec4(0.0);
    }
    
    vec3 noise_data = hash33(vec3(fragCoord + vec2(5.0), map_settings.r));
    float density = noise_data.r * 2.0 - 1.0;
    
    return vec4(
        density,
        0.0,
        0.0,
        0.0
    );
}



void main()
{
    ivec2 addr = ivec2(fragCoordUV * iResolution.xy);
    vec2 fragCoord = fragCoordUV * iResolution.xy;
    
    vec4 map = texelFetch(BUFFER_MAP_STATE, addr, 0);
    
    if (read_data(BUFFER_STATE, ADDR_RESET).r != 0.0) {
        //Need to regenerate the map
        vec4 map_settings = read_data(BUFFER_STATE, ADDR_MAP_SETTINGS);
        map = gen_map(fragCoord, map_settings);
    }
    
        
    if (!any(greaterThan(addr, MAP_SIZE))) {
        vec4 player_state = read_data(BUFFER_STATE, ADDR_PLAYER_STATE);
        vec3 player_position, player_velocity;
        float health, shoot;
        unpack_player(player_state, player_position, player_velocity, health, shoot); 
        
        vec2 delta;
        ivec2 player_co = map_coord_to_texel(player_position.xy, delta);
        
        vec2 blast_vec = vec2(player_co - addr) + (delta);
        float r2 = dot(blast_vec, blast_vec);
        
        
        if (shoot == 0.0 && r2 < 1.0) {
        
           map -= vec4(0.3, 0.0, 0.0, 0.0);
        }
        
    }
    
    
    
    fragColor = map;
}
