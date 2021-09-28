// Cavefly Render pass. Creates the final output


vec4 render_map(vec2 map_coords) {
    vec4 distance_field = sample_map_distance_field(BUFFER_MAP_STATE, ShapeTexture, map_coords);
    
    float sdf = distance_field.b - 0.5;
    
    vec4 noise = texture(NoiseTexture, map_coords * 0.25) - 0.5;
    
    float sdf_main = sdf + dot(noise, vec4(
        0.8, // Large distortions
        0.0, // Grass
        0.075, // Dirt Clods
        0.1 // Rocks
    ));
    
    float sdf_grass = sdf + abs((noise.r + 0.5) * (noise.g + 0.5) * 0.4);
    
    float base = 1.0 - smoothstep(-0.01, 0.01, sdf_main);
    float grass = 1.0 - smoothstep(-0.01, 0.3, sdf_grass);
    
    
    return vec4(base, base * grass, 0.0, 0.0);
}


vec4 render_background(vec2 map_coords, vec2 light_coords) {
    vec2 v = light_coords - map_coords;
    vec4 noise = texture(BackgroundTexture, map_coords * 0.1) - 0.5;
    float shadows = dot(noise.xy, v);
    
    float falloff = 1.0 / (dot(v, v) * 1.0 + 1.0);
    
    float light = falloff;
    light += (falloff * shadows);
    
    return vec4(light);
}



void main(){
    
    vec2 camera_position = vec2(3.0);
    camera_position.x += sin(iTime);
    
    vec2 map_viewport = fragCoordUV * vec2(MAP_SIZE / 3) + camera_position;
    vec2 background_viewport = fragCoordUV * vec2(MAP_SIZE / 2) + camera_position;
    
    
    
    vec4 background = render_background(background_viewport, vec2(5.0));
    vec4 map = render_map(map_viewport);
    
    
    //vec4 map_state = get_sprite_rot(ShapeTexture, 4.0, vec2(0.0, 0.0), iTime, fragCoordUV);
    
    fragColor = background * map.g;
}
