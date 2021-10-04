/// Cavefly Render pass. Creates the final output


vec4 render_background(vec2 map_coords, vec2 light_coords) {
    vec2 v = light_coords - map_coords;
    vec4 noise = texture(BackgroundTexture, map_coords * 0.1) - 0.5;
    float shadows = dot(noise.xy, v);
    
    float falloff = 1.0 / (dot(v, v) * 1.0 + 1.0);
    
    float light = falloff;
    light += (falloff * shadows);
    
    return vec4(light);
}




float hash13(vec3 p3)
{
	p3  = fract(p3 * .1031);
    p3 += dot(p3, p3.zyx + 31.32);
    return fract((p3.x + p3.y) * p3.z);
}

vec2 shadowSample(vec2 prev_sample, vec2 uv, float zoom, float mip) {
    float n1 = clamp(hash13(vec3(fragCoordUV * 1000.0, iTime + mip)), 0.0, 1.0) - 0.5;
    float n2 = clamp(hash13(vec3(fragCoordUV * 1000.0, iTime * 2.0 + mip)), 0.0, 1.0) - 0.5;
    
    float coord_mip_size = mip * 0.02;
    
    vec2 coord = (uv - 0.5) * (1.0 - zoom * 0.125) + 0.5 + coord_mip_size * vec2(n1, n2);
    

    vec4 map_sample = textureLod(BUFFER_MAP_SCREEN, coord, mip);
    prev_sample.g += map_sample.r * prev_sample.r;
    prev_sample.r *= map_sample.r;
    
    return prev_sample;
}


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
    
    vec2 background_viewport = uv_to_camera_view(fragCoordUV, BUFFER_STATE, 0.8);
    vec4 background = render_background(background_viewport, vec2(1.0));
    vec4 map = textureLod(BUFFER_MAP_SCREEN, fragCoordUV, 0.0);
    
    vec2 shadow = vec2(1.0, 0.0);
    shadow = shadowSample(shadow, fragCoordUV, 4.0, 6.0);
    shadow = shadowSample(shadow, fragCoordUV, 3.0, 4.0);
    shadow = shadowSample(shadow, fragCoordUV, 2.0, 2.0);
    shadow = shadowSample(shadow, fragCoordUV, 1.0, 2.0);
    shadow.g /= 4.0;
    shadow.g = pow(shadow.g, 0.5);
    
    vec4 player_state = read_data(BUFFER_STATE, ADDR_PLAYER_STATE);
    vec3 player_position, player_velocity;
    float flame, fuel;
    unpack_player(player_state, player_position, player_velocity, flame, fuel); 
    vec4 ship_sprite = sample_ship(background_viewport, player_position);
    
    
    
    vec4 out_color = vec4(0.0);
    out_color = background;
    
    out_color *= map.g; // Map
    out_color *= shadow.r; // Shadows
    out_color += shadow.g * 0.1; // God Rays
    
    
    out_color *= step(ship_sprite.b, 0.5); // Ship Triangle
    out_color += ship_sprite.r * ship_sprite.r * flame; // Ship Flame
    
    fragColor = out_color;
}


