// Trails

vec4 min_preserve(vec4 existing, vec4 new) {
    // combines sdf preserving other channels:
    // X - data
    // Y - data
    // Z - data
    // W - sdf
    if (existing.w < new.w) {
        return existing;
    } else {
        return new;
    }
}


float draw_trail(vec2 world_coords, ship_t ship) {
    if (!ship.flame) {
        return 1.0;
    }
    vec2 ship_world_coords = ship.position.xy;
    
    float point_direction = ship.position.z;
    float c = cos(point_direction);
    float s = sin(point_direction);
    
    vec2 flame_position = ship_world_coords + vec2(s*0.1, -c*0.1);
    
    float dist = length(world_coords - flame_position);
    return dist * 0.1;
}



void main()
{
    
    if (iTime < STARTING_DELAY || fragCoord.x > TRAIL_MAP_SIZE.x || fragCoord.y > TRAIL_MAP_SIZE.y) {
        fragColor = vec4(1.0);
        return;
    }
    vec2 raw_uv = fragCoord / iResolution.xy;
    vec2 uv = mod(fragCoord, TRAIL_MAP_SIZE) / TRAIL_MAP_SIZE;
    vec2 centered = (uv - 0.5) * 2.0;
    centered.x *= TRAIL_MAP_SIZE.x / TRAIL_MAP_SIZE.y;
    vec2 world_coords = centered * MAP_SCALE;
    
    
    vec4 sdf = texture(CHANNEL_TRAILS, raw_uv);
    
    sdf = clamp(sdf + iTimeDelta / TRAIL_FADE_TIME, 0.0, 1.0); // Fade to white

    ship_t ship_1 = unpack_ship(read_data(CHANNEL_STATE, A_SHIP_1));
    ship_t ship_2 = unpack_ship(read_data(CHANNEL_STATE, A_SHIP_2));
    ship_t ship_3 = unpack_ship(read_data(CHANNEL_STATE, A_SHIP_3));
    ship_t ship_4 = unpack_ship(read_data(CHANNEL_STATE, A_SHIP_4));
    
    sdf = min_preserve(sdf, vec4(COLOR_SHIP_1.rgb, draw_trail(world_coords, ship_1)));
    sdf = min_preserve(sdf, vec4(COLOR_SHIP_2.rgb, draw_trail(world_coords, ship_2)));
    sdf = min_preserve(sdf, vec4(COLOR_SHIP_3.rgb, draw_trail(world_coords, ship_3)));
    sdf = min_preserve(sdf, vec4(COLOR_SHIP_4.rgb, draw_trail(world_coords, ship_4)));
    
    fragColor = sdf;
    //fragColor = sample_map(iChannel2, iChannelResolution[2].xy, world_coords);
    //fragColor = vec4(uv, 0.0, 1.0);
}