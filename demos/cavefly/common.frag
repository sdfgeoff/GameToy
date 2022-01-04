///////////////////////// CONSTANTS //////////////////////////////
const float PI = 3.14159;

const vec3 SHIP_DAMPING = vec3(2.0, 2.0, 10.0);
const vec3 SHIP_ACCELERATION = vec3(5.0, 5.0, 100.0);
const vec3 SHIP_GRAVITY = vec3(0.0, -2.0, 0.0);
const float SHIP_BOUNCE = 0.5;


const int KEY_LEFT = 37;
const int KEY_UP   = 38;
const int KEY_RIGHT = 39;
const int KEY_DOWN = 40;
const int KEY_ESC = 27;


const int MAP_HEIGHT = 14;
const int MAP_WIDTH = 14;
const ivec2 MAP_SIZE = ivec2(MAP_WIDTH, MAP_HEIGHT);

#define NUM_LIGHTS 3


// R = frames since reset
// G = map seed
const ivec2 ADDR_RESET = ivec2(0,0);

const ivec2 ADDR_CAMERA_POSITION = ivec2(3,0);
const ivec2 ADDR_PLAYER_STATE = ivec2(4,0);

const ivec2 ADDR_MAP_METADATA = MAP_SIZE + ivec2(2);


const float MAP_SCREEN_SCALE = 1.0; // (In Theory) Extend the map screen buffer by this percent to allow better god-rays at screen edge
const float LIGHT_DISTANCE_SCALE = 5.0;
const float CAMERA_DEFAULT_ZOOM = 3.0;


#define ENABLE_LIGHTING
//#define ENABLE_GOD_RAYS
//#define VIEW_WHOLE_MAP


//////////////////////////// STATE MANAGEMENT //////////////////////////

// Fetch a single pixel from the state buffer buffer
vec4 read_data(sampler2D buffer, ivec2 address){
    return texelFetch(buffer, address, 0);
}



// Packs the player data into a vec4
vec4 pack_player(vec3 position, vec3 velocity, float flame, float fuel) {
    position = position / vec3(vec2(MAP_SIZE + 2), PI);
    velocity = velocity / vec3(vec2(MAP_SIZE + 2), PI);
    return vec4(
        uintBitsToFloat(packSnorm2x16(position.xy)),
        uintBitsToFloat(packSnorm2x16(velocity.xy)),
        uintBitsToFloat(packSnorm2x16(vec2(position.z, velocity.z))),
        uintBitsToFloat(packSnorm2x16(vec2(flame, fuel)))
    );
}


// Unpacks the player data from a vec4
void unpack_player(in vec4 data, out vec3 position, out vec3 velocity, out float flame, out float fuel) {
    position.xy = unpackSnorm2x16(floatBitsToUint(data.x));
    velocity.xy = unpackSnorm2x16(floatBitsToUint(data.y));
    vec2 angle_data = unpackSnorm2x16(floatBitsToUint(data.z));
    vec2 extra_data = unpackSnorm2x16(floatBitsToUint(data.w));
    position.z = angle_data.x;
    velocity.z = angle_data.y;
    
    position *= vec3(vec2(MAP_SIZE + 2), PI);
    velocity *= vec3(vec2(MAP_SIZE + 2), PI);
    
    flame = extra_data.x;
    fuel = extra_data.y;
}


vec4 pack_map_metadata(vec2 start_position, vec2[NUM_LIGHTS] light_array) {
    return vec4(
        uintBitsToFloat(packHalf2x16(start_position)),
        uintBitsToFloat(packHalf2x16(light_array[0])),
        uintBitsToFloat(packHalf2x16(light_array[1])),
        uintBitsToFloat(packHalf2x16(light_array[2]))
    );
}
void unpack_map_metadata(vec4 data, out vec2 start_position, out vec2[NUM_LIGHTS] light_array) {
    start_position = unpackHalf2x16(floatBitsToUint(data.x));
    light_array[0] = unpackHalf2x16(floatBitsToUint(data.y));
    light_array[1] = unpackHalf2x16(floatBitsToUint(data.z));
    light_array[2] = unpackHalf2x16(floatBitsToUint(data.a));

}

vec2 uv_to_camera_view_raw(vec2 uv, vec2 camera_position, float z) {
    uv -= 0.5;
    uv.x *= iResolution.x / iResolution.y;
    uv += 0.5;
    uv -= 0.5;
    uv = uv * z + camera_position;
    return uv;
}

vec2 uv_to_camera_view(vec2 uv, sampler2D state_buffer, float z_offset) {
    vec4 cam_data = read_data(state_buffer, ADDR_CAMERA_POSITION);
    return uv_to_camera_view_raw(uv, cam_data.xy, cam_data.z - z_offset);
}


/////////////////////////// UTIL ///////////////////////

float tlerp(float start, float end, float time_constant, float dt) {
    float t = exp(-dt / time_constant);
    return mix(end, start, t);
}
vec4 tlerp(vec4 start, vec4 end, float time_constant, float dt) {
    float t = exp(-dt / time_constant);
    return mix(end, start, t);
}


////////////////////////// SPRITE SAMPLING /////////////////////////////




vec4 get_sprite_rot(sampler2D sprites, float num_rows, vec2 tile_id, float angle, vec2 delta) {
    float s = sin(angle);
    float c = cos(angle);
    mat2 rot = mat2(c, -s, s, c);
    delta = (rot * (delta - 0.5) * 0.99) + 0.5;
    vec2 coords = (tile_id + delta) / num_rows;
    return texture(sprites, coords);
}

vec4 get_sprite(sampler2D sprites, vec2 num_rows, vec2 tile_id, vec2 delta) {
    if (any(greaterThan(abs(delta), vec2(1.0)))) {
        return vec4(0.0);
    }
    delta = ((delta - 0.5) * 0.99) + 0.5;
    delta = delta * 0.5 + 0.5;
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

