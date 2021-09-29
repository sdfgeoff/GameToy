// Cavefly Render pass. Creates the final output


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
    vec4 map = texture(BUFFER_MAP_SCREEN, fragCoordUV);
    
    
    //vec4 map_state = get_sprite_rot(ShapeTexture, 4.0, vec2(0.0, 0.0), iTime, fragCoordUV);
    
    fragColor = background * map.g;
}
