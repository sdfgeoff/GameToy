void main() {
    vec2 uv_dist = FragCoordUV + sin(iTime) * cos(FragCoordUV.x * 50.0) * 0.01;
    col_out = texture(col_in, uv_dist);
}
