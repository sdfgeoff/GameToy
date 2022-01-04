// Cavefly Map Buffer
//
// Red channel - raw
// Green Tile: tile_type (how many corners are "full" + position of those corners)
// Blue Tile: tile_rotation (how many 90 degree rotations are required to make this tile line up properly)


// Hash function from https://www.shadertoy.com/view/4djSRW
float hash13(vec3 p3)
{
	p3  = fract(p3 * .1031);
    p3 += dot(p3, p3.zyx + 31.32);
    return fract((p3.x + p3.y) * p3.z);
}


bool cavex(ivec2 coord, float seed) {
    coord -= 1;
    if (any(greaterThan(coord, MAP_SIZE - 1)) || any(lessThan(coord, ivec2(0)))) {
        return true;
    }

    ivec2 delta = ivec2(0, 1);
    bool noise_here = hash13(vec3(coord, seed)) > 0.5;
    bool noise_above = hash13(vec3(coord + delta.xy, seed)) > 0.5;
    bool noise_below = hash13(vec3(coord - delta.xy, seed)) > 0.5;
    bool noise_left = hash13(vec3(coord + delta.yx, seed)) > 0.5;
    bool noise_right = hash13(vec3(coord - delta.yx, seed)) > 0.5;
    
    bool magic = (noise_right == noise_left &&
        noise_above == noise_below &&
        noise_above != noise_right);
    
    return noise_here && !magic;
}


vec4 gen_map(ivec2 coord, float seed) {
    ivec2 delta = ivec2(0, 1);
    bool here = cavex(coord, seed);
    bool above = cavex(coord + delta, seed);
    bool right = cavex(coord + delta.yx, seed);
    bool above_right = cavex(coord + delta.yy, seed);
    
    int tile_type = int(here) + int(above) + int(right) + int(above_right);
    int tile_rot = 0;
    
    // There's probably a better way to do this stack, but because it only runs
    // at map-generation-time it isn't so critical.
    if (tile_type == 1) {
        if (here) {
            tile_rot = 0;
        } else if (above) {
            tile_rot = 3;
        } else if (above_right) {
            tile_rot = 2;
        } else if (right) {
            tile_rot = 1;
        }
    } else if (tile_type == 2) {
        if (here == above_right) {
            tile_type = 6;
            if (here) {
                tile_rot = 1;
            } else {
                tile_rot = 0;
            }
        } else {
            if (here && above) {
                tile_rot = 0;
            } else if (above && above_right) {
                tile_rot = 3;
            } else if (right && above_right) {
                tile_rot = 2;
            } else {
                tile_rot = 1;
            }
        }
    } else if (tile_type == 3) {
        if (!here) {
            tile_rot = 2;
        } else if (!above) {
            tile_rot = 1;
        } else if (!above_right) {
            tile_rot = 0;
        } else if (!right) {
            tile_rot = 3;
        }
    }
    
    
    if (tile_type == 6 && hash13(vec3(coord, seed + 0.5)) > 0.5) {
        tile_type = 5;
        tile_rot += 1;
    }
    
    return vec4(
        float(here),
        float(tile_type),
        float(tile_rot),
        0.0
    );
}



ivec2 find_light_position(const ivec2 start_position, ivec2 initial_direction, float seed, float id) {
    int bounces_remaining = 10;
    
    ivec2 current_position = start_position;
    ivec2 travel_direction = initial_direction;
    
    
    while (bounces_remaining >= 0) {
        if (cavex(current_position + travel_direction, seed) == false) {
            // Free Space
            current_position = current_position + travel_direction;
        } else {
            int sign = hash13(vec3(current_position, seed + id + float(bounces_remaining))) < 0.5 ? -1 : 1;
            
            // GLSL doesn't have imat2's, so:
            
            travel_direction = ivec2(
                dot(vec2(travel_direction), vec2(0, -sign)),
                dot(vec2(travel_direction), vec2(sign, 0))
            );
            
            bounces_remaining -= 1;
        }
    }
    
    return current_position;
     
}


void main()
{
    ivec2 addr = ivec2(fragCoord);
    
    vec4 map = texelFetch(BUFFER_MAP_STATE, addr, 0);
    vec4 reset_state = read_data(BUFFER_STATE, ADDR_RESET);
    
    if (reset_state.r == 0.0) {
        //Need to regenerate the map
        float seed = reset_state.g;
        
        if (addr == ADDR_MAP_METADATA) {
            ivec2 start_position = ivec2(1,MAP_HEIGHT/2);
            while (
                cavex(start_position, seed) == true ||  // Clear here
                cavex(start_position + ivec2(0,1), seed) == true || // Clear above
                cavex(start_position + ivec2(0,-1), seed) == false // Ground Below
            ) {
                start_position.x += 1;
                if (start_position.x > MAP_WIDTH) {
                    start_position.y += 1;
                    start_position.x = 0;
                }
            }
            
            map = pack_map_metadata(
                vec2(start_position),
                vec2[NUM_LIGHTS](
                    vec2(find_light_position(start_position, ivec2(0, 1), seed, 0.0)),
                    vec2(find_light_position(start_position, ivec2(1, 0), seed, 1.0)),
                    vec2(find_light_position(start_position, ivec2(-1, 0), seed, 2.0))
                )
            );
        } else {
            map = gen_map(addr, seed);
        }
    }
    

    fragColor = map;
}
