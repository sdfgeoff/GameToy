// Cavefly shared functions

const ivec2 MAP_SIZE = ivec2(14, 14);


const ivec2 ADDR_RESET = ivec2(0,0);
const ivec2 ADDR_MAP_SETTINGS = ivec2(1,0);
const ivec2 ADDR_CAMERA_POSITION = ivec2(3,0);


// Fetch a single pixel from the state buffer buffer
vec4 read_data(sampler2D buffer, ivec2 address){
    return texelFetch(buffer, address, 0);
}


vec2 uv_to_camera_view(vec2 uv, sampler2D state_buffer, float z) {
    uv -= 0.5;
    uv.x *= iResolution.x / iResolution.y;
    uv += 0.5;
    
    vec4 cam_data = read_data(state_buffer, ADDR_CAMERA_POSITION);
    uv = uv * cam_data.z + cam_data.xy;
    return uv;
}


vec4 get_sprite_rot(sampler2D sprites, float num_rows, vec2 tile_id, float angle, vec2 delta) {
    float s = sin(angle);
    float c = cos(angle);
    mat2 rot = mat2(c, -s, s, c);
    delta = (rot * (delta - 0.5) * 0.99) + 0.5;
    vec2 coords = (tile_id + delta) / num_rows;
    return texture(sprites, coords);
}

vec4 get_sprite(sampler2D sprites, vec2 num_rows, vec2 tile_id, vec2 delta) {
    delta = ((delta - 0.5) * 0.99) + 0.5;
    vec2 coords = (tile_id + delta) / num_rows;
    return texture(sprites, coords);
}



vec4 sample_map_distance_field(sampler2D map_state_buffer, sampler2D shape_texture, vec2 map_coords) {
    ivec2 addr = ivec2(map_coords);
    if (any(greaterThan(map_coords, vec2(MAP_SIZE) + 1.0)) || any(lessThan(map_coords, vec2(0.0)))) {
        return vec4(1.0);
    }
    
    vec2 delta = map_coords - vec2(addr);
    
    vec4 map_state = texelFetch(map_state_buffer, addr, 0);
    
    ivec2 centered_addr = ivec2(round(map_coords));
    //float tile_ = texelFetch(map_state_buffer, centered_addr, 0).r;
    float tile_offset = (7.0 - map_state.g);
    float rot = map_state.b * 3.14159 / 2.0;
    
    
    
    vec4 tile = get_sprite_rot(shape_texture, 8.0, vec2(6.0, tile_offset), rot, delta);
    
    return tile;
}

