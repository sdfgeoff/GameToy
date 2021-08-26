/////////////// GAMEPLAY SETTINGS //////////////////

const float STARTING_DELAY = 3.0; // Time at start before race begins
const float ZOOM = 0.7; // Zooms the viewport

const vec3 SHIP_THRUST = vec3(10.0, 10.0, 40.0); // Note: global ref frame for x/y
const vec3 SHIP_DAMPING = vec3(2.0, 2.0, 8.0);   // Note: global ref frame for x/y

const float GROUND_FRICTION = 5.0;  // Deceleration from colliding with world

const float MAP_SCALE = 10.0; // Size of the map in world coordinates
const float SHIP_SCALE = 0.1; // Radius/size of the ship in world coordinates


const vec3 MAP_START_LINE = vec3(vec2(0.5, 0.707) * MAP_SCALE, -1.45); // Where to place the ships


const int KEY_LEFT = 37;
const int KEY_UP   = 38;
const int KEY_RIGHT = 39;
const int KEY_DOWN = 40;


/////////////// PERFORMANCE SETTINGS //////////////////
const float SPRITE_SIZE = 64.0;            // Size of a sprite in the sprite sheet
const vec2 MAP_SIZE = vec2(256, 128);       // Size of the sprite sheet to use for the map
const vec2 TRAIL_MAP_SIZE = vec2(512, 256); // Size of the trail buffer


/////////////// ADRESSES_IN_BUFFERS //////////////////
const ivec2 SPRITE_SHIP = ivec2(0,0);
const ivec2 SPRITE_FLAME = ivec2(1,0);
const ivec2 SPRITE_START_BOX = ivec2(2,0);

const ivec2 A_SHIP_1 = ivec2(0,0);
const ivec2 A_SHIP_2 = ivec2(0,1);
const ivec2 A_SHIP_3 = ivec2(1,0);
const ivec2 A_SHIP_4 = ivec2(1,1);



////////////// Visual Settings /////////////////

// Colors for the different ships
const vec4 COLOR_SHIP_1 = vec4(0.0, 0.6, 1.0, 1.0);
const vec4 COLOR_SHIP_2 = vec4(1.0, 0.6, 0.0, 1.0);
const vec4 COLOR_SHIP_3 = vec4(0.0, 1.0, 0.6, 1.0);
const vec4 COLOR_SHIP_4 = vec4(1.0, 0.0, 0.6, 1.0);

// Seconds for the image to fade to white. Due to other visual effects
// it visually fades a long time before this
const float TRAIL_FADE_TIME = 120.0; 

vec4 read_data(sampler2D buffer, ivec2 address){
    return texelFetch(buffer, address, 0);
}



////////////////// FUNCTIONS FOR SHIP STATE /////////////////
/* Each ship has a bunch of attributes. These all need to be
preserved each frame. The struct ship_t contains the attributes
and the pack_ship and unpack_ship functions are used to
stuff the ship into a vec4 color for storage */


struct ship_t {
    vec3 position;
    vec3 velocity;
    bool flame;
};

    
vec4 pack_ship(ship_t ship) {
	uint pos = packHalf2x16(ship.position.xy);
    uint velocity = packHalf2x16(ship.velocity.xy);
    uint angular = packHalf2x16(vec2(ship.position.z, ship.velocity.z));
    
    uint flags = uint(ship.flame);
    // Can possibly add other things such as engine being on into the flags
    
    return vec4(
        uintBitsToFloat(pos),
        uintBitsToFloat(velocity),
        uintBitsToFloat(angular),
        uintBitsToFloat(flags)
       );
}

ship_t unpack_ship(vec4 data) {
    uint pos = floatBitsToUint(data.x);
    uint velocity = floatBitsToUint(data.y);
    uint angular = floatBitsToUint(data.z);
    uint flags = floatBitsToUint(data.w);
    
    bool flame = bool(flags);
    
    vec2 ang = unpackHalf2x16(angular);
    
    return ship_t(
    	vec3(unpackHalf2x16(pos), ang.x),
        vec3(unpackHalf2x16(velocity), ang.y),
        flame
    );
}


////////////// SAMPLING FUNCTIONS ////////////////
/* Data stored in the buffers often has transforms
applied to it, both in where it's stored and the
data itself. These functions provide ergonomic access to
data in other buffers. */

vec4 sample_map(sampler2D map_channel, vec2 map_channel_resolution, vec2 world_coords) {
    vec2 uv = world_coords;
    uv.x *= MAP_SIZE.y / MAP_SIZE.x;
    
    uv /= MAP_SCALE;
    uv = uv * 0.5 + 0.5;
    
    if (any(lessThan(uv, vec2(0.0))) || any( greaterThan(uv, vec2(1.0)))) {
        return vec4(1.0);
    }
    
    uv = uv * MAP_SIZE / map_channel_resolution;
    
    vec4 raw = texture(map_channel, uv);
    raw.xyzw -= 0.5;
    
    raw.w *= 3.14 * 4.0;
    
    return raw;
}

vec4 sample_trails(sampler2D trail_channel, vec2 trail_channel_resolution, vec2 world_coords) {
    vec2 uv = world_coords;
    uv.x *= TRAIL_MAP_SIZE.y / TRAIL_MAP_SIZE.x;
    uv /= MAP_SCALE;
    uv = uv * 0.5 + 0.5;
    
    if (any(lessThan(uv, vec2(0.0))) || any( greaterThan(uv, vec2(1.0)))) {
        return vec4(1.0);
    }
    uv = uv * TRAIL_MAP_SIZE / trail_channel_resolution;
    
    vec4 raw = texture(trail_channel, uv);

    return raw;
}
