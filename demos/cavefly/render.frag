/// Cavefly Render pass. Creates the final output

#define NUM_LIGHTS 5

vec4 render_background(vec2 map_coords, vec3[NUM_LIGHTS] light_array) {
    vec4 noise = texture(BackgroundTexture, map_coords * 0.1) - 0.5;
    float light = 0.0;
    
    for (int i=0; i<NUM_LIGHTS; i+=1) {
        vec3 light_data = light_array[i];
        vec2 v = light_data.xy - map_coords;
        float falloff = light_data.z / (dot(v, v) * LIGHT_DISTANCE_SCALE + 1.0);
        
        float shadows = max(dot(noise.xyz, vec3(v, 1.0)), 0.0);
        
        light += falloff * shadows + falloff;
    }
    return vec4(light);
}


float hash13(vec3 p3)
{
	p3  = fract(p3 * .1031);
    p3 += dot(p3, p3.zyx + 31.32);
    return fract((p3.x + p3.y) * p3.z);
}

vec2 shadowSample(vec2 prev_sample, vec2 uv, vec2 viewport, vec3 light_data, float zoom) {
    float n1 = clamp(hash13(vec3(fragCoordUV * 1000.0, iTime + zoom)), 0.0, 1.0);
    
    vec2 v = (light_data.xy - viewport);
    v.x *= iResolution.y / iResolution.x;
    float lv = length(v);
    float mip = min(lv * zoom * 1.0, 5.0) + 2.0;
    vec2 coord = uv + v * (zoom + lv * n1 * 6.0) * 0.02;
    
    vec4 map_sample = textureLod(BUFFER_MAP_SCREEN, coord, mip);
    
    float falloff = 1.0 / (dot(v, v) * LIGHT_DISTANCE_SCALE + 1.0);
    
    prev_sample.g += map_sample.r * falloff;//map_sample.r * prev_sample.r * falloff;
    prev_sample.r *= mix(1.0, map_sample.r, falloff);
    return prev_sample;
}


// TODO:
// Evaluate perf of godray map sampling vs map calculating
// Clamp max god-ray length to improve perf
// PHYSICS!!!! MAP GENERATION

vec4 sample_ship(vec2 world_coords, vec3 player_position) {
    vec2 pos = world_coords - player_position.xy;
    float s = sin(player_position.z);
    float c = cos(player_position.z);
    
    mat2 rotmat = mat2(
        c, -s,
        s, c
    );
    
    vec2 coords = rotmat * pos;
    coords.y *= -0.5;
    coords *= 10.0;
    
    vec4 ship = get_sprite(ShapeTexture, vec2(4, 2), vec2(0,1), coords);
    
    return ship;
};



void main(){
    
    vec3 light_array[NUM_LIGHTS] = vec3[NUM_LIGHTS](
        // X, Y, Brightness
        vec3(1.0, 1.0, 0.8),
        vec3(1.0,4.0, 0.8),
        vec3(1.0,8.0, 0.8),
        vec3(5.0,1.0, 0.8),
        vec3(4.0,4.0, 0.8)
    );

    
    vec2 background_viewport = uv_to_camera_view(fragCoordUV, BUFFER_STATE, 1.0);
    vec4 background = render_background(background_viewport, light_array);
    
    
    
    
    vec2 shadow = vec2(1.0, 0.0);
    
    vec2 map_screen_uv = (fragCoordUV) * (1.0 / MAP_SCREEN_SCALE);
    vec4 map = textureLod(BUFFER_MAP_SCREEN, map_screen_uv, 0.0);
    vec2 map_viewport = uv_to_camera_view(fragCoordUV, BUFFER_STATE, 1.0);
    
    for (int i=0; i<NUM_LIGHTS; i+=1) {
        vec3 light_data = light_array[i];
        
        vec2 v = (light_data.xy - map_viewport);// / 3.0 * 1.2;
        
        shadow = shadowSample(shadow, map_screen_uv, map_viewport, light_data, 0.0);
        //shadow = shadowSample(shadow, map_screen_uv, map_viewport, light_data, 3.0);
        

        shadow.g /= 1.0;
    }
    shadow.g = pow(shadow.g, 0.5);
    
    vec4 player_state = read_data(BUFFER_STATE, ADDR_PLAYER_STATE);
    vec3 player_position, player_velocity;
    float flame, fuel;
    unpack_player(player_state, player_position, player_velocity, flame, fuel); 
    vec4 ship_sprite = sample_ship(background_viewport, player_position);
    
    
    
    vec4 out_color = vec4(0.0);
    out_color = background;
    
    out_color *= map.g; // Map
    //out_color *= shadow.r * sin(iTime); // Shadows
    out_color += shadow.g * 0.3; // God Rays
    
    
    out_color *= step(ship_sprite.b, 0.5); // Ship Triangle
    out_color += ship_sprite.r * ship_sprite.r * flame; // Ship Flame
    
    fragColor = out_color;
}


