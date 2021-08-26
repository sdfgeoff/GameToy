
#define CHANNEL_STATE iChannel0
#define CHANNEL_SPRITES iChannel1
#define CHANNEL_SPRITES_RESOLUTION iChannelResolution[1].xy

#define CHANNEL_TRAILS iChannel2
#define CHANNEL_TRAILS_RESOLUTION iChannelResolution[2].xy

//#define SHOW_SPRITE_BUFFER
//#define SHOW_STATE_BUFFER
//#define SHOW_TRAILS_BUFFER


vec4 neon(float sdf, vec4 color, float glow_width) {
    float ramp = clamp(1.0 - sdf / glow_width, 0.0, 1.0);
    vec4 outp = vec4(0.0);
    ramp = ramp * ramp;
    outp += pow(color, vec4(4.0)) * ramp;
    ramp = ramp * ramp;
    outp += color * ramp;
    ramp = ramp * ramp;
    outp += vec4(1.0) * ramp;
    return outp;
}


mat3 get_trans_mat(float angle, vec2 position, vec2 scale) {
    // Constructs a transformation matrix
    float c = cos(angle);
    float s = sin(angle);
    
    mat3 offset = mat3(
        c, s, 0.0,
        -s, c, 0.0,
        position.x, position.y, 1.0
    ) * mat3(
        scale.x, 0.0, 0.0,
        0.0, scale.y, 0.0,
        0.0, 0.0, 1.0
    );
    
    return offset;
}



vec4 fetch_sprite(ivec2 tile_id, vec2 uv) {
    if (any(lessThan(uv, vec2(-1.0))) || any( greaterThan(uv, vec2(1.0)))) {
        return vec4(1.0);
    }
    uv = uv * 0.5 + 0.5 + vec2(tile_id);
    uv = uv * SPRITE_SIZE; // Convert to pixels;
    uv.y += MAP_SIZE.y;
    uv /= CHANNEL_SPRITES_RESOLUTION; // Convert back to UV;
    return texture(CHANNEL_SPRITES, uv);
}


vec4 draw_sprite(vec2 frag_coord, ivec2 tile_id, mat3 trans_mat) {
    vec2 uv = (inverse(trans_mat) * vec3(frag_coord, 1.0)).xy;
    return fetch_sprite(tile_id, uv);
}


float draw_background(vec2 world_coordinates) {
    vec2 sections = mod(world_coordinates, 1.0);
    vec2 lines = abs(0.5 - sections) + 0.04;
    
    return min(lines.x, lines.y);
}


vec4 draw_start_box(vec2 world_coords) {
    mat3 offset = get_trans_mat(
        MAP_START_LINE.z, // angle
        MAP_START_LINE.xy, // position
        vec2(0.5) // scale
    );
    
    float sdf = draw_sprite(
        world_coords,
        SPRITE_START_BOX,
        offset
    ).b;
    return neon(sdf, vec4(1.0, 1.0, 1.0, 1.0), 0.05) * 0.2;
}


float draw_map(vec2 world_coords, float line_width) {
    vec4 raw = sample_map(CHANNEL_SPRITES, CHANNEL_SPRITES_RESOLUTION, world_coords);
    
    float edge_sdf = abs(raw.z) / line_width;
    
    if (raw.z > 0.0) {
        edge_sdf = min(edge_sdf, draw_background(world_coords));
    }
    
    return edge_sdf;
}


vec4 draw_ship(vec2 world_coords, ship_t ship, vec4 color) {
    mat3 ship_trans = get_trans_mat(
        ship.position.z, // angle
        ship.position.xy, // position
        vec2(SHIP_SCALE) // scale
    );
    
    mat3 flame_trans = ship_trans * mat3(
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, -1.45, 1.0
    );
    
    vec4 final_sdf = draw_sprite(
        world_coords,
        SPRITE_SHIP,
        ship_trans
    );
    
    
    if (ship.flame) {
        final_sdf = min(
            final_sdf,
            draw_sprite(
            	world_coords,
        		SPRITE_FLAME,
            	flame_trans
        	)
    	);
    }
    
    
    return neon(final_sdf.z, color, 0.1);
}




void mainImage( out vec4 fragColor, in vec2 fragCoord )
{   
    
    vec2 uv = fragCoord/iResolution.xy;
    vec2 centered = (uv - 0.5) * 2.0;
    centered.x*= iResolution.x / iResolution.y; // (0,0) in center of screen
    vec2 world_coords = centered;
    world_coords /= ZOOM;
    
    fragColor = vec4(0.0);
    
    ship_t ship_1 = unpack_ship(read_data(CHANNEL_STATE, A_SHIP_1));
    ship_t ship_2 = unpack_ship(read_data(CHANNEL_STATE, A_SHIP_2));
    ship_t ship_3 = unpack_ship(read_data(CHANNEL_STATE, A_SHIP_3));
    ship_t ship_4 = unpack_ship(read_data(CHANNEL_STATE, A_SHIP_4));
    
    
    world_coords += ship_1.position.xy;
    
    float predict_amount = dot(ship_1.velocity.xy, ship_1.velocity.xy);
    world_coords += ship_1.velocity.xy * predict_amount * 0.008;
    
    vec4 ship_1_sprite = draw_ship(world_coords, ship_1, COLOR_SHIP_1);
    vec4 ship_2_sprite = draw_ship(world_coords, ship_2, COLOR_SHIP_2);
    vec4 ship_3_sprite = draw_ship(world_coords, ship_3, COLOR_SHIP_3);
    vec4 ship_4_sprite = draw_ship(world_coords, ship_4, COLOR_SHIP_4);
        
    vec4 map = neon(
        draw_map(world_coords, 0.03),
        vec4(0.9, 0.9, 0.9, 1.0), 0.1
    );
    
    vec4 trail_data = sample_trails(CHANNEL_TRAILS, CHANNEL_TRAILS_RESOLUTION, world_coords);
    vec4 trails = neon(
        -0.005 + trail_data.a,
        vec4(trail_data.rgb, 1.0), 0.05
    ) * 0.1;
    
    vec4 start_box = draw_start_box(world_coords);
    
    
    //vec4 map_data = sample_map(CHANNEL_MAP, CHANNEL_MAP_RESOLUTION, world_coords);
    //fragColor += vec4(map_data.rgb, 0.0);
    
    //vec4 trail_data = sample_trails(CHANNEL_TRAILS, CHANNEL_TRAILS_RESOLUTION, world_coords);
    
    fragColor += map;
    fragColor += trails;
    fragColor += start_box;
    
    fragColor += ship_1_sprite;
    fragColor += ship_2_sprite;
    fragColor += ship_3_sprite;
    fragColor += ship_4_sprite;
    
    
    
    if (iTime < STARTING_DELAY) {
    	float dot1 = length(centered + vec2(0.5, -0.5)) - 0.1;
        float dot2 = length(centered + vec2(0.0, -0.5)) - 0.1;
    	float dot3 = length(centered + vec2(-0.5, -0.5)) - 0.1;
        
        vec4 red = vec4(1.0, 0.0, 0.0, 1.0);
        vec4 green = vec4(0.0, 1.0, 0.0, 1.0);
        vec4 col1 = green;
        vec4 col2 = mix(red, green, float(iTime > (STARTING_DELAY / 3.0)));
        vec4 col3 = mix(red, green, float(iTime > (STARTING_DELAY / 3.0 * 2.0)));
    	
    	fragColor += neon(dot1, col1, 0.1);
    	fragColor += neon(dot2, col2, 0.1);
    	fragColor += neon(dot3, col3, 0.1);
    }
    

#ifdef SHOW_SPRITE_BUFFER
    fragColor = texture(CHANNEL_SPRITES, uv);
#endif
#ifdef SHOW_STATE_BUFFER
    fragColor = texture(CHANNEL_STATE, uv);
#endif
#ifdef SHOW_TRAILS_BUFFER
    fragColor = texture(CHANNEL_TRAILS, uv);
#endif

}
