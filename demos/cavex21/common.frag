const float PI = 3.14159;

// ------------------------ CONFIGURATION -------------------------


// Keyboard Controls
const int KEY_LEFT = 37;
const int KEY_UP   = 38;
const int KEY_RIGHT = 39;
const int KEY_DOWN = 40;
const int KEY_SHOOT = 32;
const int KEY_ESC = 27;

// Map configuration
const ivec2 MAP_SIZE = ivec2(32, 32);

// Viewport configuration
const float CAM_ZOOM = 2.0;
const float CAM_ZOOM_SCALING = 0.2;
const float CAM_DAMPING = 0.99;


// Ship flight characteristics
const vec3 SHIP_DAMPING = vec3(2.0, 2.0, 10.0);
const vec3 SHIP_ACCELERATION = vec3(5.0, 5.0, 100.0);
const float SHIP_TERRAIN_HITBOX_SIZE = 0.05;
const float SHIP_TERRAIN_BOUNCE_ELASTICITY = 0.6;

// Ship graphical characteristics
const float SHIP_GRAPHICAL_SIZE = 0.1;
const vec2 SHIP_NOSE = vec2(0.0, 0.8) * SHIP_GRAPHICAL_SIZE;
const vec2 SHIP_LEFT_WING = vec2(0.5, -0.5) * SHIP_GRAPHICAL_SIZE;
const vec2 SHIP_RIGHT_WING = vec2(-0.5, -0.5) * SHIP_GRAPHICAL_SIZE;

// Bullet settings
const float BULLET_RELOAD_TIME_SEC = 0.2;
//const float BULLET_LIFE_TIME = 1.0;
//const float MAX_BULLETS = BULLET_LIFE_TIME / BULLET_RELOAD_TIME_SEC;

// ------------------------- ADDRESSES ----------------------------
//
// Addresses into the state buffer. To be used in association 
// with the read_data function


// ADDR_RESET: red channel set to non-zero when the game is being reset
// This is used to regenerate the map, reset player etc.
const ivec2 ADDR_RESET = ivec2(0,0);

// ADDR_MAP_SETTINGS: Configure the map generator. Note this does not immediatley
// regenerate the map - the map will regenerate based on ADDR_RESET
//   - r = seed
const ivec2 ADDR_MAP_SETTINGS = ivec2(0,1);

// ADDR_CAMERA_POSITION: Stores where the camera currently is
//   - x = x position
//   - y = y position
//   - z = zoom
const ivec2 ADDR_CAMERA_POSITION = ivec2(0,2);

// ADDR_PLAYER_STATE: Stores state of the player
// Use the pack/unpack functions to access this data
//   - x = x and y position
//   - y = x and y velocity
//   - z = angular position and velocity
//   - a = health and ammo state

const ivec2 ADDR_PLAYER_STATE = ivec2(0,3);




// Fetch a single pixel from the state buffer buffer
vec4 read_data(sampler2D buffer, ivec2 address){
    return texelFetch(buffer, address, 0);
}



// Packs the player data into a vec4
vec4 pack_player(vec3 position, vec3 velocity, float health, float shoot) {
    position = position / vec3(vec2(MAP_SIZE), PI);
    velocity = velocity / vec3(vec2(MAP_SIZE), PI);
    return vec4(
        uintBitsToFloat(packSnorm2x16(position.xy)),
        uintBitsToFloat(packSnorm2x16(velocity.xy)),
        uintBitsToFloat(packSnorm2x16(vec2(position.z, velocity.z))),
        uintBitsToFloat(packSnorm2x16(vec2(health, shoot)))
    );
}


// Unpacks the player data from a vec4
void unpack_player(in vec4 data, out vec3 position, out vec3 velocity, out float health, out float shoot) {
    position.xy = unpackSnorm2x16(floatBitsToUint(data.x));
    velocity.xy = unpackSnorm2x16(floatBitsToUint(data.y));
    vec2 angle_data = unpackSnorm2x16(floatBitsToUint(data.z));
    vec2 extra_data = unpackSnorm2x16(floatBitsToUint(data.w));
    position.z = angle_data.x;
    velocity.z = angle_data.y;
    
    position *= vec3(vec2(MAP_SIZE), PI);
    velocity *= vec3(vec2(MAP_SIZE), PI);
    
    health = extra_data.x;
    shoot = extra_data.y;
}



// ------------------------- MAP -------------------------------


vec4 sample_map_raw(sampler2D map_buffer, ivec2 coord) {
    bvec4 bounds = bvec4(
        greaterThan(coord, MAP_SIZE-1),
        lessThan(coord, ivec2(0))
    );
    
    if (any(bounds)) {
        return vec4(1.0);
    } else {
        return texelFetch(map_buffer, coord, 0);
    }
    

}

// Converts a coordinate into a texel and a position within
// the texel

ivec2 map_coord_to_texel(vec2 co, out vec2 delta) {
    // Coordinate 0,0 should be the middle of the map
    co += vec2(MAP_SIZE) / 2.0 - 1.0; 
    vec2 co_int = floor(co);
    ivec2 here_co = ivec2(co_int);
    delta = co - co_int;
    return here_co;
}



// The underlying map representation, used for both physics and rendering
//    - r = normalX
//    - g = normalY
//    - a = dirt presence/absense
vec4 map_base(sampler2D map_buffer, in vec2 co) {
    
    // Find out which grid ID we are in, and where within that tile we are
    vec2 delta;
    ivec2 here_co = map_coord_to_texel(co, delta);
    vec2 neg_delta = vec2(1.0) - delta;
    
    
    // Sample all directions so we can smooth between them
    ivec2 d = ivec2(0, 1);
    mat2 samples = mat2(
        sample_map_raw(map_buffer, here_co).r,
        sample_map_raw(map_buffer, here_co + d.yx).r,
        sample_map_raw(map_buffer, here_co + d.xy).r,
        sample_map_raw(map_buffer, here_co + d.yy).r
    );
    
    // Contain some numbers for ease of future referencing
    vec4 x = vec4(neg_delta.x, delta.x, -1.0, 1.0);
    vec4 y = vec4(neg_delta.y, delta.y, -1.0, 1.0);
    
    // Partial derivatives in X/Y direction
    vec2 normal = vec2(
        dot(x.zw * samples, y.xy),
        dot(x.xy * samples, y.zw)
    );
    
    // Bilinear interpolation
    float density = dot(x.xy * samples, y.xy);
    
    // Approx sdf
    float sdf = density / pow(dot(normal, normal), 0.25);
    
    return vec4(
        normalize(normal),
        sdf,
        density
    );
}
