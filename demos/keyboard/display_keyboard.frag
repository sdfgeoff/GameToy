const int KEY_LEFT  = 37;
const int KEY_UP    = 38;
const int KEY_RIGHT = 39;
const int KEY_DOWN  = 40;


const float INNER_SPACING = 0.1;
const float OUTER_SPACING = 0.4;
const float ARROW_WIDTH = 0.02;
const float HEAD_WIDTH = 0.05;

const float LINE_WIDTH = 0.005;



// The arrow goes from a to b. It's thickness is w1. The arrow
// head's thickness is w2.
float sdArrow( in vec2 p, vec2 a, vec2 b, float w1, float w2 )
{
	// The MIT License
	// Copyright © 2021 Inigo Quilez
	// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions: The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software. THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

	// Distance to an arrow

	// List of some other 2D distances: https://www.shadertoy.com/playlist/MXdSRf
	//
	// and www.iquilezles.org/www/articles/distfunctions2d/distfunctions2d.htm


	// return min(length(p-a)-w1,length(p-b)); for debugging

	// constant setup
	const float k = 3.0;   // arrow head ratio
	vec2  ba = b - a;
	float l2 = dot(ba,ba);
	float l = sqrt(l2);

	// pixel setup
	p = p-a;
	p = mat2(ba.x,-ba.y,ba.y,ba.x)*p/l;
	p.y = abs(p.y);
	vec2 pz = p-vec2(l-w2*k,w2);

	// === distance (four segments) ===
	vec2 q = p;
	q.x -= clamp( q.x, 0.0, l-w2*k );
	q.y -= w1;
	float di = dot(q,q);
	//----
	q = pz;
	q.y -= clamp( q.y, w1-w2, 0.0 );
	di = min( di, dot(q,q) );
	//----
	if( p.x<w1 ) // conditional is optional
	{
	q = p;
	q.y -= clamp( q.y, 0.0, w1 );
	di = min( di, dot(q,q) );
	}
	//----
	if( pz.x>0.0 ) // conditional is optional
	{
	q = pz;
	q -= vec2(k,-1.0)*clamp( (q.x*k-q.y)/(k*k+1.0), 0.0, w2 );
	di = min( di, dot(q,q) );
	}

	// === sign ===

	float si = 1.0;
	float z = l - p.x;
	if( min(p.x,z)>0.0 ) //if( p.x>0.0 && z>0.0 )
	{
	  float h = (pz.x<0.0) ? w1 : z/k;
	  if( p.y<h ) si = -1.0;
	}
	return si*sqrt(di);
}


float edge(float sdf, float thickness) {
	return abs(sdf) - thickness;
}


float draw_key(int key, float arrow) {
	float pressed = texelFetch(keyboard, ivec2(key, 0), 0).x;
	float key_edge = texelFetch(keyboard, ivec2(key, 1), 0).x;
	float toggle = texelFetch(keyboard, ivec2(key, 2), 0).x;

	float outp = 0.0;
	outp += step(edge(arrow-0.01, 0.005), 0.0) * 0.5;

	outp += step(arrow, 0.0) * pressed;
	outp += step(edge(arrow, LINE_WIDTH), 0.0) * toggle;
	outp += step(edge(arrow - LINE_WIDTH*2.0, LINE_WIDTH), 0.0) * key_edge;

	return outp;
}




void main() {
	col.r = texture(keyboard, FragCoordUV).r;

	vec3 edge_vec = vec3(0.0, INNER_SPACING, OUTER_SPACING);

	vec2 uv = FragCoordUV * 2.0 - 1.0;
	uv.x *= iResolution.x / iResolution.y;

	float up_arrow = sdArrow(uv, edge_vec.xy, edge_vec.xz, ARROW_WIDTH, HEAD_WIDTH);
	float down_arrow = sdArrow(uv, -edge_vec.xy, -edge_vec.xz, ARROW_WIDTH, HEAD_WIDTH);
	float right_arrow = sdArrow(uv, edge_vec.yx, edge_vec.zx, ARROW_WIDTH, HEAD_WIDTH);
	float left_arrow = sdArrow(uv, -edge_vec.yx, -edge_vec.zx, ARROW_WIDTH, HEAD_WIDTH);

	col.g = 0.0;
	col.g += draw_key(KEY_LEFT, left_arrow);
	col.g += draw_key(KEY_RIGHT, right_arrow);
	col.g += draw_key(KEY_UP, up_arrow);
	col.g += draw_key(KEY_DOWN, down_arrow);

	col.b = 0.0;
	col.a = 0.0;
		//col.g = 0.0;//step(0.0, left_arrow);
}
