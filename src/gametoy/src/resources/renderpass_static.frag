#version 300 es
precision highp float;
uniform vec3 iResolution; // viewport resolution (in pixels)
uniform float iTime;      // shader playback time (in seconds)
uniform float iTimeDelta; // render time (in seconds)
uniform uint iFrame;      // shader playback frame
uniform vec4 iDate;       // (year, month, day, time in seconds)
//uniform vec4 iMouse;      // mouse pixel coords. xy: current (if MLB down), zw: click
in vec2 FragCoordUV;
