// Sprites


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


void main()
{
    draw_sprites(fragColor, fragCoord);
}
