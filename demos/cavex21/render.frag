float TRIANGLE_sign(vec2 p1, vec2 p2, vec2 p3)
    // Pinched from https://www.shadertoy.com/view/4s3fzj
{
    return (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y);
}

bool TRIANGLE(vec2 vertices[3], vec2 uv)
    // Pinched from https://www.shadertoy.com/view/4s3fzj
{
    bool b1 = TRIANGLE_sign(uv, vertices[0], vertices[1]) < 0.0f;
    bool b2 = TRIANGLE_sign(uv, vertices[1], vertices[2]) < 0.0f;
    bool b3 = TRIANGLE_sign(uv, vertices[2], vertices[0]) < 0.0f;
    
    return (b1 == b2) && (b2 == b3);
}





void main()
{
    // Normalized pixel coordinates (from 0 to 1)
    vec2 fragCoord = fragCoordUV * iResolution.xy;
    vec2 uv = fragCoord/iResolution.xy;

    // Figure out screen coordinates with the camera
    vec2 coords = uv * 2.0 - 1.0;
    coords.x *= iResolution.x / iResolution.y;
    vec4 cam_data = read_data(BUFFER_STATE, ADDR_CAMERA_POSITION);
    vec2 cam_pos = cam_data.xy;
    float cam_zoom = cam_data.z;
    
    coords *= cam_zoom;
    coords += cam_pos;
    
    
    // Draw the player
    vec4 player_state = read_data(BUFFER_STATE, ADDR_PLAYER_STATE);
    vec3 player_position, player_velocity;
    float health, shoot;
    unpack_player(player_state, player_position, player_velocity, health, shoot); 
    
    float s = sin(player_position.z);
    float c = cos(player_position.z);
    mat2 ori = mat2(
        c, s,
        -s, c
    );
    vec2 VERTS[3] = vec2[](
        player_position.xy + ori * SHIP_NOSE,
        player_position.xy + ori * SHIP_LEFT_WING,
        player_position.xy + ori * SHIP_RIGHT_WING
    );
    vec4 player = vec4(TRIANGLE(VERTS, coords));
    
    // Draw the map
    vec4 map_data = map_base(BUFFER_MAP_STATE, coords);
    
    float map_shape = map_data.a;
    float map_islands = smoothstep(-0.01, 0.01, map_shape);
    
    float map_shadows = pow(map_shape, 0.2) * (0.5 - map_islands) * 2.0 + map_islands;
    float map = map_islands + map_shadows * 0.5;
    vec4 map_col = vec4(map * 0.5);
    
    
    
    // Draw bullets
    float bullets = 0.0;
    int i = 0;
    for (i = 0; i <= MAX_BULLETS; i+=1) {
        vec4 bullet_data = read_data(BUFFER_STATE, ADDR_BULLET_START + ivec2(0, i));
        
        vec3 pos, velocity;
        float age;
        unpack_bullet(bullet_data, pos, velocity, age);
        if (age != 0.0) {
            bullets += step(length(coords - pos.xy), 0.02);
        }
    }
    
    
    
    fragColor = vec4(1.0);
    
    
    fragColor *= map_col;
    
    
    fragColor = mix(fragColor, vec4(0.8, 0.8, 0.2, 1.0), bullets);
    fragColor = mix(fragColor, vec4(0.4, 0.4, 1.0, 1.0), player);
    
}
