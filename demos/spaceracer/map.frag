// The map. The map is in coords (0,0) to MAP_SIZE



void draw_map(out vec4 fragColor, in vec2 fragCoord) {
    vec2 map_uv = mod(fragCoord, MAP_SIZE) / MAP_SIZE;
    
    vec2 coords = (map_uv - 0.5) * 2.0;
    coords.x *= MAP_SIZE.x / MAP_SIZE.y;
    
    
    vec2 line = line_segment(coords, vec2(-0.5, 0.0), vec2(0.5, 0.0));
    
    
    
    float angle = atan(coords.x, coords.y);
    
    float track_radius = 0.7;
    
    // Distort
    track_radius += cos(angle * 3.0 - 1.0) * 0.10;
    track_radius += cos(angle * 5.0 + 1.0) * 0.10;
    
    float track_width = 0.05;
    
    float center_sdf = abs(track_radius - length(line));
    float border_sdf = center_sdf - track_width;
    float boundary_sdf = abs(track_width - center_sdf);
    
    vec2 border_norm = normalize(vec2(
        dFdx(border_sdf),
        dFdy(border_sdf)
    ));
    
    fragColor = vec4(border_norm + 0.5, border_sdf + 0.5, 0.0);
}


void main()
{
    draw_map(fragColor, fragCoord);    
}
