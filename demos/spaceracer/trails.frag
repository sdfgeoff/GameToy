// Sprites and the map. The map is in coords (0,0) to MAP_SIZE
// and the sprites sit on top of it from (0, MAP_SIZE)


vec2 line_segment(vec2 point, vec2 segment_start, vec2 segment_end) {
    // Returns a vector pointing to the line segment.
    vec2 line_direction = segment_start - segment_end;
    vec2 point_on_line = segment_end;
    float segment_length = length(line_direction);


    float projected_distance = dot(normalize(line_direction), point - point_on_line);
    vec2 closest_point = point_on_line + projected_distance * line_direction / segment_length;

    float distance_from_end = -projected_distance;
    float distance_from_start = projected_distance - segment_length;

    // Rounded caps on segment
    if (distance_from_start > 0.0) {
        closest_point = segment_start;
    }
    if (distance_from_end > 0.0) {
        closest_point = segment_end;
    }

    return point - closest_point;
}


vec4 n_union(vec4 existing, vec2 new) {
    // Converts a normal towards a surface into:
    // X - normal X
    // Y - normal Y
    // Z - sdf squared
    // W - vacant
    float sdf_squared_new = dot(new, new);
    if (sdf_squared_new < existing.z){
        return vec4(new, sdf_squared_new, 0.0);
    }
    return existing;
}


vec4 draw_sprite_start_box(vec2 uv) {
    // Start line and places for ships. Normaldata invalid
    uv.x -= 0.5;

    
    vec4 outp = vec4(0.0, 0.0, 9999.0, 0.0);
    
    // Start/finish line
    outp = n_union(outp, line_segment(uv, vec2(-0.4, 0.9), vec2(0.4, 0.9)));
    outp = n_union(outp, line_segment(uv, vec2(-0.4, 0.95), vec2(0.4, 0.95)));
    
    // Duplicate start boxes early
    uv.x = abs(uv.x);
    uv.y = mod(uv.y, 0.3);
    
    outp = n_union(outp, line_segment(uv, vec2(0.1, 0.18), vec2(0.2, 0.18)));
    outp = n_union(outp, line_segment(uv, vec2(0.2, 0.15), vec2(0.2, 0.18)));
    outp = n_union(outp, line_segment(uv, vec2(0.1, 0.15), vec2(0.1, 0.18)));
    
    outp.b = sqrt(outp.b);

    return outp;
}


vec4 draw_sprite_ship(vec2 uv){
    // Ship
    const float SCALE = 1.5;
    uv = (uv - 0.5);
    
    uv.y += 0.175;
    uv *= SCALE;
    
    vec2 unmirror = uv;
    uv.x = abs(uv.x); // Mirror
    
    vec4 outp = vec4(0.0, 0.0, 9999.0, 0.0);
    
    // Wings
    outp = n_union(outp, line_segment(uv, vec2(0.0, 0.75), vec2(0.5, 0.0)));
    outp = n_union(outp, line_segment(uv, vec2(0.15, -0.35), vec2(0.5, 0.0)));
    outp = n_union(outp, line_segment(uv, vec2(0.15, -0.35), vec2(0.15, 0.25)));
    outp = n_union(outp, line_segment(uv, vec2(0.0, 0.5), vec2(0.15, 0.25)));
    
    // Engine
    outp = n_union(outp, line_segment(uv, vec2(0.15, -0.2), vec2(0.0, -0.25)));
    outp = n_union(outp, line_segment(uv, vec2(0.11, -0.22), vec2(0.06, -0.35)));
    outp = n_union(outp, line_segment(uv, vec2(0.0, -0.35), vec2(0.06, -0.35)));
    
    // Cockpit
    outp = n_union(outp, line_segment(uv, vec2(0.1, 0.2), vec2(0.0, 0.35)));
    outp = n_union(outp, line_segment(uv, vec2(0.1, 0.2), vec2(0.1, 0.1)));
    outp = n_union(outp, line_segment(uv, vec2(0.0, 0.15), vec2(0.1, 0.1)));
    
    outp.b = sqrt(outp.b) / SCALE;
    if (unmirror.x < 0.0) {
        outp.x *= -1.0;
    }
    return outp;
}

vec4 draw_sprite_flame(vec2 uv){
    // Engine flame. Note, normaldata invalid and SDF does not have unit gradient.
    uv = (uv - 0.5);

    uv.y -= 0.25;
    
    if (uv.y < 0.0) {
        // Stretch and taper the coordinate system
    	uv.y /= (1.0 - uv.y) * 4.0;
        uv.x /= (1.0 + 10.0 * uv.y);
    }
    
    vec4 outp = vec4(0.0, 0.0, 9999.0, 0.0);
    outp = n_union(outp, line_segment(uv, vec2(-0.025, 0.0), vec2(0.025, 0.0)));
    
    outp.b = sqrt(outp.b);

    return outp;
}



void draw_map(out vec4 fragColor, in vec2 fragCoord) {
    vec2 map_uv = mod(fragCoord, MAP_SIZE) / MAP_SIZE;
    
    vec2 coords = (map_uv - 0.5) * 2.0;
    coords.x *= MAP_SIZE.x / MAP_SIZE.y;
    
    
    vec2 line = line_segment(coords, vec2(-0.5, 0.0), vec2(0.5, 0.0));
    
    
    
    float angle = atan(coords.x, coords.y);
    
    float track_radius = 0.7;
    
    // Distort
    track_radius += cos(angle * 3.0 - 1.0) * 0.10;
    track_radius += cos(angle * 5.0 + 1.0) * 0.10;
    
    float track_width = 0.05;
    
    float center_sdf = abs(track_radius - length(line));
    float border_sdf = center_sdf - track_width;
    float boundary_sdf = abs(track_width - center_sdf);
    
    vec2 border_norm = normalize(vec2(
        dFdx(border_sdf),
        dFdy(border_sdf)
    ));
    
    fragColor = vec4(border_norm + 0.5, border_sdf + 0.5, 0.0);
}


void draw_sprites(out vec4 fragColor, in vec2 fragCoord) {
    vec2 tile_uv = mod(fragCoord, SPRITE_SIZE) / SPRITE_SIZE;
    ivec2 tile_id = ivec2(fragCoord / SPRITE_SIZE);
    
    vec4 outp = vec4(0.0);
    
    if (tile_id == SPRITE_SHIP) {
        outp = draw_sprite_ship(tile_uv);
    } else if (tile_id == SPRITE_FLAME) {
        outp = draw_sprite_flame(tile_uv);
    } else if (tile_id == SPRITE_START_BOX) {
        outp = draw_sprite_start_box(tile_uv);
    }
    
    fragColor = outp;
}


void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    ivec2 map_id = ivec2(fragCoord / MAP_SIZE);
    if (map_id == ivec2(0,0)) {
        draw_map(fragColor, fragCoord);
        return;
    } else {
        vec2 start_position = fragCoord;
        start_position.y -= MAP_SIZE.y;
        draw_sprites(fragColor, start_position);
    	return;
	}
    
}
