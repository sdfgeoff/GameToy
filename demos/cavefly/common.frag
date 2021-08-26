// Cavefly shared functions

const ivec2 MAP_SIZE = ivec2(14, 14);


const ivec2 ADDR_RESET = ivec2(0,0);
const ivec2 ADDR_MAP_SETTINGS = ivec2(1,0);


// Fetch a single pixel from the state buffer buffer
vec4 read_data(sampler2D buffer, ivec2 address){
    return texelFetch(buffer, address, 0);
}
