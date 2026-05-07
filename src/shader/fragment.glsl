#version 330 core

uniform ivec2 viewport;
uniform float zoom;
uniform vec2 center;

vec2 complex_multiply(vec2 left, vec2 right) {
    return vec2(left.x * right.x - left.y * right.y, left.x * right.y + left.y * right.x);
}

out vec4 outColor;

void main()
{
    int iterations = int(64.0 * (log(zoom) + 1.0));
    vec2 c = vec2((2.0 * gl_FragCoord.x / float(viewport.x) - 1.0) / zoom + center.x, (2.0 * gl_FragCoord.y / float(viewport.y) - 1.0) / zoom + center.y);
    vec2 z = vec2(0.0f, 0.0f);
    for (int i = 0; i < iterations; i++) {
        z = complex_multiply(z, z) + c;
        if (dot(z, z) > 4.0f) {
            outColor = vec4(float(i) / float(iterations), 0.0, 0.0, 1.0f);
            return;
        }
    }
    outColor = vec4(1.0, 1.0, 1.0, 1.0f);
}
