// Cavefly Render pass. Creates the final output




vec4 get_sprite_rot(sampler2D sprites, float num_rows, vec2 tile_id, float angle, vec2 delta) {
    float s = sin(angle);
    float c = cos(angle);
    mat2 rot = mat2(c, -s, s, c);
    
    delta = (rot * (delta - 0.5)) + 0.5;
    
    vec2 coords = (tile_id + delta) / num_rows;
    
    return texture(sprites, coords);
}



vec4 draw_map_shape(vec2 map_coords) {
    vec4 outp = vec4(0.0);
    
    ivec2 addr = ivec2(map_coords);
    vec2 delta = map_coords - vec2(addr);
    
    vec4 map_state = texelFetch(BUFFER_MAP_STATE, addr, 0);
    
    ivec2 centered_addr = ivec2(round(map_coords));
    outp.r = texelFetch(BUFFER_MAP_STATE, centered_addr, 0).r;

    
    float tile_offset = (7.0 - map_state.g);
    float rot = map_state.b * 3.14159 / 2.0;
    
    
    
    vec4 tile = get_sprite_rot(ShapeTexture, 8.0, vec2(6.0, tile_offset), rot, delta);
    
    outp.b = tile.b;
    
    return outp;
}



void main(){
    
    
    vec4 map_state = draw_map_shape(fragCoordUV * vec2(MAP_SIZE));
    map_state = vec4(step(0.5, map_state.b));
    
    //vec4 map_state = get_sprite_rot(ShapeTexture, 4.0, vec2(0.0, 0.0), iTime, fragCoordUV);
    
    fragColor = map_state;
}
