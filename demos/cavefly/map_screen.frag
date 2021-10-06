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

void main(){
    vec2 map_viewport = uv_to_camera_view(fragCoordUV * MAP_SCREEN_SCALE, BUFFER_STATE, 1.2);
    vec4 map = render_map(map_viewport);
    
    fragColor = map;
}
